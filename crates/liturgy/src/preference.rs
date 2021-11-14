use std::collections::HashMap;

pub type PreferenceKey = String;

pub type PreferenceValue = String;

pub trait ClientPreferences {
    fn value(&self, key: &PreferenceKey) -> PreferenceValue;
}

impl ClientPreferences for [(PreferenceKey, PreferenceValue); 0] {
    fn value(&self, key: &PreferenceKey) -> PreferenceValue {
        todo!()
    }
}

impl ClientPreferences for HashMap<PreferenceKey, PreferenceValue> {
    fn value(&self, key: &PreferenceKey) -> PreferenceValue {
        self.get(key)
            .cloned()
            // TODO — fallbacks from liturgy's default preferences
            .unwrap_or_else(|| String::from(""))
    }
}
