use liturgy::GloriaPatri;

lazy_static! {
    pub static ref GLORIA_PATRI: GloriaPatri = GloriaPatri::from((
        "Glory to the Father, and to the Son, ",
        "and to the Holy Spirit: ",
        "as it was in the beginning, is now,",
        "and will be for ever. Amen. "
    ));
}
