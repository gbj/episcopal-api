#[macro_use]
extern crate rocket;

use rocket::fairing::{Fairing, Info, Kind};
use rocket::fs::{relative, FileServer};
use rocket::http::Header;
use rocket::{Request, Response};

mod calendar;
mod document;
mod psalm;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(CORS)
        .mount("/static", FileServer::from(relative!("static")))
        .mount("/calendar", routes![calendar::day])
        .mount(
            "/document",
            routes![
                document::doc_to_json,
                document::doc_to_html,
                psalm::psalm_by_number,
                psalm::psalms_by_citation
            ],
        )
}
pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _req: &'r Request<'_>, res: &mut Response<'r>) {
        res.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        res.set_header(Header::new("Access-Control-Allow-Methods", "GET"));
        res.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        res.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}
