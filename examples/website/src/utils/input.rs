use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlInputElement};

pub fn value(event: Event) -> String {
    event
        .target()
        .unwrap()
        .unchecked_into::<HtmlInputElement>()
        .value()
}
