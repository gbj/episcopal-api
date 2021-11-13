pub type PreferenceKey = String;

pub type PreferenceValue = String;

pub trait ClientPreferences {
    fn value(&self, key: &PreferenceKey) -> &PreferenceValue;
}
