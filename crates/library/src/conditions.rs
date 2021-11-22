use calendar::Season;
use liturgy::{Condition, PreferenceKey, PreferenceValue};

lazy_static! {
    /// True when it is not Lent (including Holy Week)
    pub static ref NOT_LENT: Condition = Condition::Not(Box::new(Condition::Or(
        Box::new(Condition::Season(Season::Lent)),
        Box::new(Condition::Season(Season::HolyWeek))
    )));

    /// True when the "Insert Gloria Patri between psalms" preference is not set
    pub static ref NOT_INSERT_GLORIA: Condition = Condition::Not(Box::new(Condition::Preference(
        PreferenceKey::from("insertGloria"),
        PreferenceValue::from("true")
    )));
}
