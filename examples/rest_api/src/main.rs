#[macro_use]
extern crate rocket;

use std::env;
use std::path::Path;

use rocket::fairing::{Fairing, Info, Kind};
use rocket::fs::{relative, FileServer};
use rocket::http::Header;
use rocket::{Request, Response};

mod calendar;
mod document;
mod error;
mod psalm;

#[launch]
fn rocket() -> _ {
    let static_dir = match env::var_os("STATIC_DIR") {
        Some(static_dir) => Path::new(&static_dir).join("examples/api/static"),
        None => Path::new(relative!("static")).join(""),
    };

    rocket::build()
        .attach(CORS)
        .mount(
            "/calendar",
            routes![calendar::day, calendar::day_with_psalms],
        )
        .mount(
            "/document",
            routes![
                document::doc_to_json,
                document::doc_to_html,
                psalm::psalm_by_number,
                psalm::psalms_by_citation
            ],
        )
        .mount("/pray", routes![document::doc_to_html])
        .mount("/", FileServer::from(static_dir))
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
