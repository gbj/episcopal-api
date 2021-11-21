use liturgy::{Content, Document};

lazy_static! {
    pub static ref GLORIA_PATRI: Document = Document::new().content(Content::Gloria(
        String::from("Glory to the Father, and to the Son, "),
        String::from("and to the Holy Spirit: "),
        String::from("as it was in the beginning, is now,"),
        String::from("and will be for ever. Amen. ")
    ));
}
