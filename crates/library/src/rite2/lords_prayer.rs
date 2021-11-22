use liturgy::{Choice, DisplayFormat, Document, Text};

lazy_static! {
    pub static ref LORDS_PRAYER_ABBREV: Choice = Choice::from([
      Document::from(
        Text::from("Our Father, who art in heaven,\n\thallowed be thy Name,\n\tthy kingdom come,\n\tthy will be done,\n\ton earth as it is in heaven.\nGive us this day our daily bread.\nAnd forgive us our trespasses,\n\tas we forgive those\n\twho trespass against us.\nAnd lead us not into temptation,\n\tbut deliver us from evil.")
          .display_format(DisplayFormat::Unison)
      ).label("Traditional"),
      Document::from(
        Text::from("Our Father in heaven,\n\thallowed be your Name,\n\tyour kingdom come,\n\tyour will be done,\n\ton earth as in heaven.\nGive us today our daily bread.\nForgive us our sins,\n\tas we forgive those\n\twho sin against us.\nSave us from the time of trial,\n\tand deliver us from evil.")
          .display_format(DisplayFormat::Unison)
      ).label("Contemporary")
    ]);
}
