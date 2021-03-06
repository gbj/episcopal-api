use std::collections::HashMap;

use liturgy::{Document, Series};

macro_rules! hash_map {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = ::std::collections::HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PageType {
    Document,
    Category(&'static str),
}

lazy_static! {
    pub static ref TABLE_OF_CONTENTS: HashMap<String, Vec<(String, PageType, Document)>> = hash_map! {
        "office".into() => vec![
            ("morning-prayer".into(), PageType::Document, library::rite2::office::MORNING_PRAYER_II.clone()),
            ("noonday-prayer".into(), PageType::Document, library::rite2::office::NOONDAY_PRAYER.clone()),
            ("compline".into(), PageType::Document, library::rite2::office::COMPLINE.clone()),
        ],
        "canticle".into() => vec![
            ("1".into(), PageType::Document, library::rite1::canticles::CANTICLE_1.clone()),
            ("2".into(), PageType::Document, library::rite1::canticles::CANTICLE_2.clone()),
            ("3".into(), PageType::Document, library::rite1::canticles::CANTICLE_3.clone()),
            ("4".into(), PageType::Document, library::rite1::canticles::CANTICLE_4.clone()),
            ("5".into(), PageType::Document, library::rite1::canticles::CANTICLE_5.clone()),
            ("6".into(), PageType::Document, library::rite1::canticles::CANTICLE_6.clone()),
            ("7".into(), PageType::Document, library::rite1::canticles::CANTICLE_7.clone()),
            ("8".into(), PageType::Document, library::rite2::canticles::CANTICLE_8.clone()),
            ("9".into(), PageType::Document, library::rite2::canticles::CANTICLE_9.clone()),
            ("10".into(), PageType::Document, library::rite2::canticles::CANTICLE_10.clone()),
            ("11".into(), PageType::Document, library::rite2::canticles::CANTICLE_11.clone()),
            ("12".into(), PageType::Document, library::rite2::canticles::CANTICLE_12.clone()),
            ("13".into(), PageType::Document, library::rite2::canticles::CANTICLE_13.clone()),
            ("14".into(), PageType::Document, library::rite2::canticles::CANTICLE_14.clone()),
            ("15".into(), PageType::Document, library::rite2::canticles::CANTICLE_15.clone()),
            ("16".into(), PageType::Document, library::rite2::canticles::CANTICLE_16.clone()),
            ("17".into(), PageType::Document, library::rite2::canticles::CANTICLE_17.clone()),
            ("18".into(), PageType::Document, library::rite2::canticles::CANTICLE_18.clone()),
            ("19".into(), PageType::Document, library::rite2::canticles::CANTICLE_19.clone()),
            ("20".into(), PageType::Document, library::rite2::canticles::CANTICLE_20.clone()),
            ("21".into(), PageType::Document, library::rite2::canticles::CANTICLE_21.clone()),
            ("12-eow".into(), PageType::Document, library::eow::canticles::CANTICLE_12_EOW.clone()),
            ("15-eow".into(), PageType::Document, library::eow::canticles::CANTICLE_15_EOW.clone()),
            ("16-eow".into(), PageType::Document, library::eow::canticles::CANTICLE_16_EOW.clone()),
            ("18-eow".into(), PageType::Document, library::eow::canticles::CANTICLE_18_EOW.clone()),
            ("21-eow".into(), PageType::Document, library::eow::canticles::CANTICLE_21_EOW.clone()),
            ("a".into(), PageType::Document, library::eow::canticles::CANTICLE_A.clone()),
            ("b".into(), PageType::Document, library::eow::canticles::CANTICLE_B.clone()),
            ("c".into(), PageType::Document, library::eow::canticles::CANTICLE_C.clone()),
            ("d".into(), PageType::Document, library::eow::canticles::CANTICLE_D.clone()),
            ("e".into(), PageType::Document, library::eow::canticles::CANTICLE_E.clone()),
            ("f".into(), PageType::Document, library::eow::canticles::CANTICLE_F.clone()),
            ("g".into(), PageType::Document, library::eow::canticles::CANTICLE_G.clone()),
            ("h".into(), PageType::Document, library::eow::canticles::CANTICLE_H.clone()),
            ("i".into(), PageType::Document, library::eow::canticles::CANTICLE_I.clone()),
            ("j".into(), PageType::Document, library::eow::canticles::CANTICLE_J.clone()),
            ("k".into(), PageType::Document, library::eow::canticles::CANTICLE_K.clone()),
            ("l".into(), PageType::Document, library::eow::canticles::CANTICLE_L.clone()),
            ("m".into(), PageType::Document, library::eow::canticles::CANTICLE_M.clone()),
            ("n".into(), PageType::Document, library::eow::canticles::CANTICLE_N.clone()),
            ("o".into(), PageType::Document, library::eow::canticles::CANTICLE_O.clone()),
            ("p".into(), PageType::Document, library::eow::canticles::CANTICLE_P.clone()),
            ("q".into(), PageType::Document, library::eow::canticles::CANTICLE_Q.clone()),
            ("r".into(), PageType::Document, library::eow::canticles::CANTICLE_R.clone()),
            ("s".into(), PageType::Document, library::eow::canticles::CANTICLE_S.clone()),
        ],
        "category".into() => vec![
            ("opening-sentences".into(), PageType::Category("Opening Sentences"), Document::from(Series::from(library::rite2::OPENING_SENTENCES.clone()))),
            ("invitatory-antiphons".into(), PageType::Category("Invitatory Antiphons"), Document::from(Series::from(library::rite2::INVITATORY_ANTIPHONS.clone()))),
            ("closing-sentences".into(), PageType::Category("Closing Sentences"), Document::from(Series::from(library::rite2::OPENING_SENTENCES.clone()))),
        ]
    };
}
