use liturgy::Document;

lazy_static! {
    pub static ref LITURGIES: Vec<(&'static str, &'static Document)> = vec![
        ("morning-prayer", &library::rite2::office::MORNING_PRAYER_II),
        ("noonday-prayer", &library::rite2::office::NOONDAY_PRAYER),
        ("compline", &library::rite2::office::COMPLINE),
    ];
}
