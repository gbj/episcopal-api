use calendar::Season;
use liturgy::Condition;

lazy_static! {
    pub static ref NOT_LENT: Condition = Condition::Not(Box::new(Condition::Or(
        Box::new(Condition::Season(Season::Lent)),
        Box::new(Condition::Season(Season::HolyWeek))
    )));
}
