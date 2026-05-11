use std::sync::Arc;

use fabro_agent::{Sandbox, shell_quote};

pub async fn detect_changed_files(sandbox: &Arc<dyn Sandbox>) -> Vec<String> {
    let diff_result = sandbox
        .exec_command("git diff --name-only", 30_000, None, None, None)
        .await;
    let untracked_result = sandbox
        .exec_command(
            "git ls-files --others --exclude-standard",
            30_000,
            None,
            None,
            None,
        )
        .await;

    let mut files: Vec<String> = Vec::new();
    if let Ok(result) = diff_result {
        if result.is_success() {
            files.extend(
                result
                    .stdout
                    .lines()
                    .filter(|line| !line.trim().is_empty())
                    .map(String::from),
            );
        }
    }
    if let Ok(result) = untracked_result {
        if result.is_success() {
            files.extend(
                result
                    .stdout
                    .lines()
                    .filter(|line| !line.trim().is_empty())
                    .map(String::from),
            );
        }
    }

    files.sort();
    files.dedup();
    files
}

pub async fn files_touched_since(
    sandbox: &Arc<dyn Sandbox>,
    files_before: &[String],
) -> (Vec<String>, Option<String>) {
    let files_after = detect_changed_files(sandbox).await;
    let files_touched: Vec<String> = files_after
        .into_iter()
        .filter(|file| !files_before.contains(file))
        .collect();

    let last_file_touched = if files_touched.is_empty() {
        None
    } else {
        let quoted_files: Vec<String> =
            files_touched.iter().map(|file| shell_quote(file)).collect();
        let cmd = format!("ls -t {} | head -1", quoted_files.join(" "));
        sandbox
            .exec_command(&cmd, 5_000, None, None, None)
            .await
            .ok()
            .and_then(|result| {
                let trimmed = result.stdout.trim().to_string();
                (result.is_success() && !trimmed.is_empty()).then_some(trimmed)
            })
    };

    (files_touched, last_file_touched)
}
