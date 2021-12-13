#[macro_use]
extern crate lazy_static;

use perseus::{define_app, ErrorPages};
use sycamore::view;

mod table_of_contents;
mod templates;
mod utils;

define_app! {
    templates: [
        crate::templates::index::get_template::<G>(),
        crate::templates::document::get_template::<G>()
    ],
    error_pages: ErrorPages::new(|url, status, err, _| {
        view! {
            p { (format!("An error with HTTP code {} occurred at '{}': '{}'.", status, url, err)) }
        }
    }),
    locales: {
        default: "en-US",
        other: ["es-ES"]
    }
}
