use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Display;

use crate::Version;
use lectionary::ReadingType;

/// An explanatory sentence or direction for the liturgy
#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum PreferenceKey {
    Global(GlobalPref),
    Local(String),
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum GlobalPref {
    BibleVersion,
    PsalterVersion,
    Lectionary,
    PsalmCycle,
    ReadingA,
    ReadingB,
    ReadingC,
    /// Whether to insert the Gloria Patri after each psalm in the Daily Office, or only at the end of the psalms
    InsertGloria,
}

impl<T> From<T> for PreferenceKey
where
    T: Display,
{
    fn from(key: T) -> Self {
        Self::Local(key.to_string())
    }
}

impl From<GlobalPref> for PreferenceKey {
    fn from(key: GlobalPref) -> Self {
        Self::Global(key)
    }
}

#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum PreferenceValue {
    Version(Version),
    Lectionary(Lectionaries),
    ReadingType(ReadingType),
    Local(String),
}

impl From<Version> for PreferenceValue {
    fn from(version: Version) -> Self {
        Self::Version(version)
    }
}

impl From<Lectionaries> for PreferenceValue {
    fn from(lectionary: Lectionaries) -> Self {
        Self::Lectionary(lectionary)
    }
}

impl From<ReadingType> for PreferenceValue {
    fn from(reading_type: ReadingType) -> Self {
        Self::ReadingType(reading_type)
    }
}

impl From<String> for PreferenceValue {
    fn from(value: String) -> Self {
        Self::Local(value)
    }
}

impl From<&str> for PreferenceValue {
    fn from(value: &str) -> Self {
        Self::Local(value.to_string())
    }
}

impl Default for PreferenceValue {
    fn default() -> Self {
        Self::Local(Default::default())
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

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum Lectionaries {
    BCP1979DailyOffice,
    BCP1979DailyOfficePsalms,
    BCP1979ThirtyDayPsalms,
    RCLTrack1,
    RCLTrack2,
}

impl Default for Lectionaries {
    fn default() -> Self {
        Self::BCP1979DailyOffice
    }
}
