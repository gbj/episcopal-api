use liturgy::{Document, GloriaPatri};

lazy_static! {
    pub static ref GLORIA_PATRI: Document = Document::from(GloriaPatri::from((
        "Glory to the Father, and to the Son, ",
        "and to the Holy Spirit: ",
        "as it was in the beginning, is now,",
        "and will be for ever. Amen. "
    )));
}
