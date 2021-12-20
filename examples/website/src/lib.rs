#[macro_use]
extern crate lazy_static;

use perseus::{define_app, ErrorPages, Plugins};
use perseus_size_opt::{perseus_size_opt, SizeOpts};
use sycamore::view;

mod components;
mod table_of_contents;
mod templates;
mod utils;

pub const API_BASE: &str = "http://127.0.0.1:8000";

define_app! {
    templates: [
        crate::templates::index::get_template::<G>(),
        crate::templates::canticle_table::get_template::<G>(),
        crate::templates::daily_readings::get_template::<G>(),
        crate::templates::document::get_template::<G>(),
        crate::templates::psalm::get_template::<G>()
    ],
    error_pages: ErrorPages::new(|url, status, err, _| {
        view! {
            p { (format!("An error with HTTP code {} occurred at '{}': '{}'.", status, url, err)) }
        }
    }),
    locales: {
        default: "en-US",
        other: []
    },
    plugins: Plugins::new().plugin(perseus_size_opt, SizeOpts::default())
}
