use liturgy::Psalm;
use psalter::bcp1979::BCP1979_PSALTER;
use rocket::serde::json::Json;

#[get("/psalm?<number>", rank = 1)]
pub fn psalm_by_number(number: u8) -> Json<Option<Psalm>> {
    Json(BCP1979_PSALTER.psalm_by_number(number).cloned())
}

#[get("/psalm?<citation>", rank = 2)]
pub fn psalms_by_citation(citation: &str) -> Json<Vec<Psalm>> {
    Json(BCP1979_PSALTER.psalms_by_citation(citation))
}
