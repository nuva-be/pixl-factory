use std::fmt;
use std::str::FromStr;

use serde::de::Error as _;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::error::AutomationValidationError;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AutomationId(String);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AutomationTriggerId(String);

impl AutomationId {
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AutomationTriggerId {
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for AutomationId {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl AsRef<str> for AutomationTriggerId {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for AutomationId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl fmt::Display for AutomationTriggerId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl TryFrom<String> for AutomationId {
    type Error = AutomationValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        validate_id(&value, false)
            .then_some(Self(value.clone()))
            .ok_or(AutomationValidationError::InvalidAutomationId(value))
    }
}

impl TryFrom<String> for AutomationTriggerId {
    type Error = AutomationValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        validate_id(&value, true)
            .then_some(Self(value.clone()))
            .ok_or(AutomationValidationError::InvalidTriggerId(value))
    }
}

impl FromStr for AutomationId {
    type Err = AutomationValidationError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::try_from(value.to_string())
    }
}

impl FromStr for AutomationTriggerId {
    type Err = AutomationValidationError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::try_from(value.to_string())
    }
}

impl Serialize for AutomationId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl Serialize for AutomationTriggerId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for AutomationId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Self::try_from(value).map_err(D::Error::custom)
    }
}

impl<'de> Deserialize<'de> for AutomationTriggerId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Self::try_from(value).map_err(D::Error::custom)
    }
}

fn validate_id(value: &str, allow_underscore: bool) -> bool {
    let bytes = value.as_bytes();
    matches!(bytes.first(), Some(first) if first.is_ascii_lowercase() || first.is_ascii_digit())
        && bytes.len() <= 63
        && bytes.iter().skip(1).all(|b| {
            b.is_ascii_lowercase()
                || b.is_ascii_digit()
                || *b == b'-'
                || (allow_underscore && *b == b'_')
        })
}

#[cfg(test)]
mod tests {
    use super::{AutomationId, AutomationTriggerId};

    #[test]
    fn automation_id_accepts_locked_format() {
        assert!("a".parse::<AutomationId>().is_ok());
        assert!("a0-b".parse::<AutomationId>().is_ok());
        assert!("0".parse::<AutomationId>().is_ok());
    }

    #[test]
    fn trigger_id_accepts_underscore_after_first_character() {
        assert!("api_1".parse::<AutomationTriggerId>().is_ok());
        assert!("a-b_c".parse::<AutomationTriggerId>().is_ok());
    }
}
