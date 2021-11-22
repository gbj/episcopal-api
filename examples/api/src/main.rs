#[macro_use]
extern crate rocket;

mod calendar;
mod document;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(CORS)
        .mount("/calendar", routes![calendar::day])
        .mount(
            "/document",
            routes![
                document::document,
                document::doc_to_html,
                document::psalm_by_number,
                document::psalms_by_citation
            ],
        )
}

use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{Request, Response};

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
