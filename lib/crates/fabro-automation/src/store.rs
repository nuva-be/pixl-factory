use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use tokio::fs::{self, OpenOptions};
use tokio::io::AsyncWriteExt as _;
use tokio::sync::RwLock;

use crate::error::{AutomationStoreError, AutomationValidationError};
use crate::id::AutomationId;
use crate::model::{
    Automation, AutomationDraft, AutomationPatch, AutomationReplace, AutomationRevision,
};

#[derive(Debug)]
pub struct AutomationStore {
    dir:   PathBuf,
    items: RwLock<BTreeMap<AutomationId, Automation>>,
}

impl AutomationStore {
    pub async fn load(dir: impl Into<PathBuf>) -> Result<Self, AutomationStoreError> {
        let dir = dir.into();
        let mut items = BTreeMap::new();

        match fs::read_dir(&dir).await {
            Ok(mut entries) => {
                while let Some(entry) = entries
                    .next_entry()
                    .await
                    .map_err(|err| AutomationStoreError::io(&dir, err))?
                {
                    let path = entry.path();
                    if path.extension().and_then(|ext| ext.to_str()) != Some("toml") {
                        continue;
                    }
                    let Some(stem) = path.file_stem().and_then(|stem| stem.to_str()) else {
                        return Err(AutomationValidationError::InvalidAutomationId(
                            path.display().to_string(),
                        )
                        .into());
                    };
                    let id = AutomationId::try_from(stem.to_string())?;
                    let bytes = fs::read(&path)
                        .await
                        .map_err(|err| AutomationStoreError::io(&path, err))?;
                    let automation = Automation::from_toml_bytes(id.clone(), &bytes)
                        .map_err(|err| AutomationStoreError::parse(&path, err))?;
                    items.insert(id, automation);
                }
            }
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => {}
            Err(err) => return Err(AutomationStoreError::io(&dir, err)),
        }

        Ok(Self {
            dir,
            items: RwLock::new(items),
        })
    }

    #[expect(
        clippy::disallowed_methods,
        reason = "Server startup loads automations before a Tokio runtime may be available."
    )]
    pub fn load_blocking(dir: impl Into<PathBuf>) -> Result<Self, AutomationStoreError> {
        let dir = dir.into();
        let mut items = BTreeMap::new();

        match std::fs::read_dir(&dir) {
            Ok(entries) => {
                for entry in entries {
                    let entry = entry.map_err(|err| AutomationStoreError::io(&dir, err))?;
                    let path = entry.path();
                    if path.extension().and_then(|ext| ext.to_str()) != Some("toml") {
                        continue;
                    }
                    let Some(stem) = path.file_stem().and_then(|stem| stem.to_str()) else {
                        return Err(AutomationValidationError::InvalidAutomationId(
                            path.display().to_string(),
                        )
                        .into());
                    };
                    let id = AutomationId::try_from(stem.to_string())?;
                    let bytes =
                        std::fs::read(&path).map_err(|err| AutomationStoreError::io(&path, err))?;
                    let automation = Automation::from_toml_bytes(id.clone(), &bytes)
                        .map_err(|err| AutomationStoreError::parse(&path, err))?;
                    items.insert(id, automation);
                }
            }
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => {}
            Err(err) => return Err(AutomationStoreError::io(&dir, err)),
        }

        Ok(Self {
            dir,
            items: RwLock::new(items),
        })
    }

    pub async fn list(&self) -> Vec<Automation> {
        self.items.read().await.values().cloned().collect()
    }

    pub async fn get(&self, id: &AutomationId) -> Option<Automation> {
        self.items.read().await.get(id).cloned()
    }

    pub async fn create(&self, draft: AutomationDraft) -> Result<Automation, AutomationStoreError> {
        let id = draft.id.clone();
        let mut items = self.items.write().await;
        if items.contains_key(&id) {
            return Err(AutomationStoreError::AlreadyExists(id));
        }

        let automation = Automation::from_draft(draft, AutomationRevision::from_bytes(b""))?;
        let automation = self.persist_with_revision(automation).await?;
        items.insert(id, automation.clone());
        Ok(automation)
    }

    pub async fn replace(
        &self,
        id: &AutomationId,
        expected: &AutomationRevision,
        draft: AutomationReplace,
    ) -> Result<Automation, AutomationStoreError> {
        let mut items = self.items.write().await;
        let current = items
            .get(id)
            .ok_or_else(|| AutomationStoreError::NotFound(id.clone()))?;
        ensure_revision(current, expected)?;

        let automation = draft.into_automation(id.clone(), AutomationRevision::from_bytes(b""))?;
        let automation = self.persist_with_revision(automation).await?;
        items.insert(id.clone(), automation.clone());
        Ok(automation)
    }

    pub async fn patch(
        &self,
        id: &AutomationId,
        expected: &AutomationRevision,
        patch: AutomationPatch,
    ) -> Result<Automation, AutomationStoreError> {
        let mut items = self.items.write().await;
        let current = items
            .get(id)
            .ok_or_else(|| AutomationStoreError::NotFound(id.clone()))?;
        ensure_revision(current, expected)?;

        let draft = patch.apply_to(current);
        let automation = draft.into_automation(id.clone(), AutomationRevision::from_bytes(b""))?;
        let automation = self.persist_with_revision(automation).await?;
        items.insert(id.clone(), automation.clone());
        Ok(automation)
    }

    pub async fn delete(
        &self,
        id: &AutomationId,
        expected: &AutomationRevision,
    ) -> Result<(), AutomationStoreError> {
        let mut items = self.items.write().await;
        let current = items
            .get(id)
            .ok_or_else(|| AutomationStoreError::NotFound(id.clone()))?;
        ensure_revision(current, expected)?;

        let path = self.path_for(id);
        match fs::remove_file(&path).await {
            Ok(()) => {}
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => {}
            Err(err) => return Err(AutomationStoreError::io(&path, err)),
        }
        items.remove(id);
        Ok(())
    }

    async fn persist_with_revision(
        &self,
        automation: Automation,
    ) -> Result<Automation, AutomationStoreError> {
        let bytes = automation
            .to_toml_bytes()
            .map_err(|err| AutomationValidationError::InvalidWorkflowSelector(err.to_string()))?;
        atomic_write(&self.dir, &self.path_for(&automation.id), &bytes).await?;
        let revision = AutomationRevision::from_bytes(&bytes);
        Ok(Automation {
            revision,
            ..automation
        })
    }

    fn path_for(&self, id: &AutomationId) -> PathBuf {
        self.dir.join(format!("{id}.toml"))
    }
}

fn ensure_revision(
    current: &Automation,
    expected: &AutomationRevision,
) -> Result<(), AutomationStoreError> {
    if &current.revision == expected {
        Ok(())
    } else {
        Err(AutomationStoreError::RevisionMismatch {
            expected: expected.clone(),
            actual:   current.revision.clone(),
        })
    }
}

async fn atomic_write(
    dir: &Path,
    final_path: &Path,
    bytes: &[u8],
) -> Result<(), AutomationStoreError> {
    fs::create_dir_all(dir)
        .await
        .map_err(|err| AutomationStoreError::io(dir, err))?;

    let temp_path = temp_path_for(dir, final_path);
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&temp_path)
        .await
        .map_err(|err| AutomationStoreError::io(&temp_path, err))?;
    let write_result = async {
        file.write_all(bytes).await?;
        file.flush().await?;
        file.sync_all().await
    }
    .await;
    if let Err(err) = write_result {
        let _ = fs::remove_file(&temp_path).await;
        return Err(AutomationStoreError::io(&temp_path, err));
    }
    drop(file);

    if let Err(err) = fs::rename(&temp_path, final_path).await {
        let _ = fs::remove_file(&temp_path).await;
        return Err(AutomationStoreError::io(final_path, err));
    }
    Ok(())
}

fn temp_path_for(dir: &Path, final_path: &Path) -> PathBuf {
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let stem = final_path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("automation.toml");
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |duration| duration.as_nanos());
    let counter = COUNTER.fetch_add(1, Ordering::Relaxed);
    dir.join(format!(".{stem}.{now}.{counter}.tmp"))
}

#[cfg(test)]
mod tests {
    use tokio::fs;

    use super::AutomationStore;
    use crate::{
        AutomationDraft, AutomationId, AutomationPatch, AutomationReplace, AutomationRevision,
    };

    fn draft(id: &str) -> AutomationDraft {
        toml::from_str(&format!(
            r#"
id = "{id}"
name = "Nightly"
description = "Runs nightly"

[target]
repository = "fabro-sh/fabro"
ref = "main"
workflow = "deps"

[[triggers]]
id = "api"
type = "api"
"#
        ))
        .expect("draft should deserialize")
    }

    fn replacement(name: &str) -> AutomationReplace {
        toml::from_str(&format!(
            r#"
name = "{name}"
enabled = true

[target]
repository = "fabro-sh/fabro"
ref = "main"
workflow = "deps"

[[triggers]]
id = "api"
type = "api"
"#
        ))
        .expect("replacement should deserialize")
    }

    #[tokio::test]
    async fn missing_directory_loads_empty_store() {
        let dir = tempfile::tempdir().expect("tempdir should be created");
        let store = AutomationStore::load(dir.path().join("automations"))
            .await
            .expect("store should load");
        assert!(store.list().await.is_empty());
    }

    #[tokio::test]
    async fn create_writes_file() {
        let dir = tempfile::tempdir().expect("tempdir should be created");
        let automation_dir = dir.path().join("automations");
        let store = AutomationStore::load(&automation_dir)
            .await
            .expect("store should load");

        let automation = store
            .create(draft("nightly"))
            .await
            .expect("automation should be created");

        let path = automation_dir.join("nightly.toml");
        let bytes = fs::read(&path).await.expect("file should exist");
        assert_eq!(automation.revision, AutomationRevision::from_bytes(&bytes));
        assert!(String::from_utf8_lossy(&bytes).contains("name = \"Nightly\""));
    }

    #[tokio::test]
    async fn replace_changes_revision() {
        let dir = tempfile::tempdir().expect("tempdir should be created");
        let store = AutomationStore::load(dir.path())
            .await
            .expect("store should load");
        let first = store
            .create(draft("nightly"))
            .await
            .expect("automation should be created");

        let second = store
            .replace(&first.id, &first.revision, replacement("Updated"))
            .await
            .expect("automation should be replaced");

        assert_ne!(first.revision, second.revision);
        assert_eq!(second.name, "Updated");
    }

    #[tokio::test]
    async fn patch_keeps_unchanged_fields() {
        let dir = tempfile::tempdir().expect("tempdir should be created");
        let store = AutomationStore::load(dir.path())
            .await
            .expect("store should load");
        let first = store
            .create(draft("nightly"))
            .await
            .expect("automation should be created");
        let patch = AutomationPatch {
            name: Some("Patched".to_string()),
            ..AutomationPatch::default()
        };

        let patched = store
            .patch(&first.id, &first.revision, patch)
            .await
            .expect("automation should be patched");

        assert_eq!(patched.name, "Patched");
        assert_eq!(patched.description.as_deref(), Some("Runs nightly"));
        assert_eq!(patched.target, first.target);
        assert_eq!(patched.triggers, first.triggers);
    }

    #[tokio::test]
    async fn stale_revision_fails() {
        let dir = tempfile::tempdir().expect("tempdir should be created");
        let store = AutomationStore::load(dir.path())
            .await
            .expect("store should load");
        let first = store
            .create(draft("nightly"))
            .await
            .expect("automation should be created");

        let result = store
            .replace(
                &first.id,
                &AutomationRevision::from_bytes(b"stale"),
                replacement("Updated"),
            )
            .await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn delete_removes_file() {
        let dir = tempfile::tempdir().expect("tempdir should be created");
        let store = AutomationStore::load(dir.path())
            .await
            .expect("store should load");
        let automation = store
            .create(draft("nightly"))
            .await
            .expect("automation should be created");
        let path = dir.path().join("nightly.toml");

        store
            .delete(&automation.id, &automation.revision)
            .await
            .expect("automation should be deleted");

        assert!(!path.exists());
        assert!(store.get(&automation.id).await.is_none());
    }

    #[tokio::test]
    async fn startup_fails_on_malformed_toml() {
        let dir = tempfile::tempdir().expect("tempdir should be created");
        fs::write(dir.path().join("nightly.toml"), "not = [toml")
            .await
            .expect("malformed file should be writable");

        let result = AutomationStore::load(dir.path()).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn invalid_filename_fails_load() {
        let dir = tempfile::tempdir().expect("tempdir should be created");
        fs::write(
            dir.path().join("Bad.toml"),
            r#"
name = "Bad"
[target]
repository = "fabro-sh/fabro"
ref = "main"
workflow = "deps"
"#,
        )
        .await
        .expect("file should be writable");

        let result = AutomationStore::load(dir.path()).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn non_toml_files_are_ignored() {
        let dir = tempfile::tempdir().expect("tempdir should be created");
        fs::write(dir.path().join("README.md"), "ignored")
            .await
            .expect("file should be writable");

        let store = AutomationStore::load(dir.path())
            .await
            .expect("store should load");

        assert!(store.list().await.is_empty());
    }

    #[tokio::test]
    async fn get_returns_created_automation_by_id() {
        let dir = tempfile::tempdir().expect("tempdir should be created");
        let store = AutomationStore::load(dir.path())
            .await
            .expect("store should load");
        let created = store
            .create(draft("nightly"))
            .await
            .expect("automation should be created");
        let id = AutomationId::try_from("nightly".to_string()).expect("id should be valid");

        assert_eq!(store.get(&id).await, Some(created));
    }
}
