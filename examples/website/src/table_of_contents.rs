use std::collections::HashMap;

use liturgy::Document;

macro_rules! hash_map {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = ::std::collections::HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}

lazy_static! {
    pub static ref TABLE_OF_CONTENTS: HashMap<String, Vec<(String, Document)>> = hash_map! {
        "office".into() => vec![
            ("morning-prayer".into(), library::rite2::office::MORNING_PRAYER_II.clone()),
            ("noonday-prayer".into(), library::rite2::office::NOONDAY_PRAYER.clone()),
            ("compline".into(), library::rite2::office::COMPLINE.clone()),
        ],
        "canticle".into() => vec![
            ("1".into(), library::rite1::canticles::CANTICLE_1.clone()),
            ("2".into(), library::rite1::canticles::CANTICLE_2.clone()),
            ("3".into(), library::rite1::canticles::CANTICLE_3.clone()),
            ("4".into(), library::rite1::canticles::CANTICLE_4.clone()),
            ("5".into(), library::rite1::canticles::CANTICLE_5.clone()),
            ("6".into(), library::rite1::canticles::CANTICLE_6.clone()),
            ("7".into(), library::rite1::canticles::CANTICLE_7.clone()),
            ("8".into(), library::rite2::canticles::CANTICLE_8.clone()),
            ("9".into(), library::rite2::canticles::CANTICLE_9.clone()),
            ("10".into(), library::rite2::canticles::CANTICLE_10.clone()),
            ("11".into(), library::rite2::canticles::CANTICLE_11.clone()),
            ("12".into(), library::rite2::canticles::CANTICLE_12.clone()),
            ("13".into(), library::rite2::canticles::CANTICLE_13.clone()),
            ("14".into(), library::rite2::canticles::CANTICLE_14.clone()),
            ("15".into(), library::rite2::canticles::CANTICLE_15.clone()),
            ("16".into(), library::rite2::canticles::CANTICLE_16.clone()),
            ("17".into(), library::rite2::canticles::CANTICLE_17.clone()),
            ("18".into(), library::rite2::canticles::CANTICLE_18.clone()),
            ("19".into(), library::rite2::canticles::CANTICLE_19.clone()),
            ("20".into(), library::rite2::canticles::CANTICLE_20.clone()),
            ("21".into(), library::rite2::canticles::CANTICLE_21.clone()),
            ("12-eow".into(), library::eow::canticles::CANTICLE_12_EOW.clone()),
            ("15-eow".into(), library::eow::canticles::CANTICLE_15_EOW.clone()),
            ("16-eow".into(), library::eow::canticles::CANTICLE_16_EOW.clone()),
            ("18-eow".into(), library::eow::canticles::CANTICLE_18_EOW.clone()),
            ("21-eow".into(), library::eow::canticles::CANTICLE_21_EOW.clone()),
            ("a".into(), library::eow::canticles::CANTICLE_A.clone()),
            ("b".into(), library::eow::canticles::CANTICLE_B.clone()),
            ("c".into(), library::eow::canticles::CANTICLE_C.clone()),
            ("d".into(), library::eow::canticles::CANTICLE_D.clone()),
            ("e".into(), library::eow::canticles::CANTICLE_E.clone()),
            ("f".into(), library::eow::canticles::CANTICLE_F.clone()),
            ("g".into(), library::eow::canticles::CANTICLE_G.clone()),
            ("h".into(), library::eow::canticles::CANTICLE_H.clone()),
            ("i".into(), library::eow::canticles::CANTICLE_I.clone()),
            ("j".into(), library::eow::canticles::CANTICLE_J.clone()),
            ("k".into(), library::eow::canticles::CANTICLE_K.clone()),
            ("l".into(), library::eow::canticles::CANTICLE_L.clone()),
            ("m".into(), library::eow::canticles::CANTICLE_M.clone()),
            ("n".into(), library::eow::canticles::CANTICLE_N.clone()),
            ("o".into(), library::eow::canticles::CANTICLE_O.clone()),
            ("p".into(), library::eow::canticles::CANTICLE_P.clone()),
            ("q".into(), library::eow::canticles::CANTICLE_Q.clone()),
            ("r".into(), library::eow::canticles::CANTICLE_R.clone()),
            ("s".into(), library::eow::canticles::CANTICLE_S.clone()),
        ]
    };
}
