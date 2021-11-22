use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Display;

/// An explanatory sentence or direction for the liturgy
#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct PreferenceKey(String);

impl<T> From<T> for PreferenceKey
where
    T: Display,
{
    fn from(key: T) -> Self {
        Self(key.to_string())
    }
}

#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct PreferenceValue(String);

impl<T> From<T> for PreferenceValue
where
    T: Display,
{
    fn from(value: T) -> Self {
        Self(value.to_string())
    }
}

impl Default for PreferenceValue {
    fn default() -> Self {
        Self(Default::default())
    }
}

pub trait ClientPreferences {
    fn value(&self, key: &PreferenceKey) -> PreferenceValue;
}

impl ClientPreferences for [(PreferenceKey, PreferenceValue); 0] {
    fn value(&self, _key: &PreferenceKey) -> PreferenceValue {
        PreferenceValue::default()
    }
}

impl ClientPreferences for HashMap<PreferenceKey, PreferenceValue> {
    fn value(&self, key: &PreferenceKey) -> PreferenceValue {
        self.get(key).cloned().unwrap_or_default()
    }
}
