use std::{convert::TryInto, time::Duration};

use perseus::is_server;
use sycamore::prelude::{ReadSignal, Signal};
use wasm_bindgen::{prelude::Closure, JsCast, UnwrapThrowExt};

pub fn window() -> Option<web_sys::Window> {
    if is_server!() {
        None
    } else {
        web_sys::window()
    }
}

pub fn document() -> Option<web_sys::Document> {
    window().and_then(|window| window.document())
}

pub fn get_element_by_id(id: &str) -> Option<web_sys::Element> {
    document().and_then(|document| document.get_element_by_id(id))
}

pub fn location() -> Option<web_sys::Location> {
    window().map(|window| window.location())
}

// Current window.location.hash without the beginning #
pub fn location_hash() -> Option<String> {
    location().and_then(|location| location.hash().ok().map(|hash| hash.replace('#', "")))
}

pub fn location_pathname() -> Option<String> {
    location().and_then(|location| location.pathname().ok())
}

pub fn window_event_signal(event_name: &'static str) -> ReadSignal<Option<web_sys::Event>> {
    let signal = Signal::new(None);
    if let Some(window) = window() {
        window
            .add_event_listener_with_callback(
                event_name,
                Closure::wrap(Box::new({
                    let signal = signal.clone();
                    move |ev: web_sys::Event| {
                        signal.set(Some(ev));
                    }
                }) as Box<dyn Fn(web_sys::Event)>)
                .into_js_value()
                .as_ref()
                .unchecked_ref(),
            )
            .unwrap_throw();
    }
    signal.handle()
}

pub fn set_timeout(cb: impl Fn() + 'static, duration: Duration) {
    if !is_server!() {
        let a = Closure::wrap(Box::new(cb) as Box<dyn Fn()>).into_js_value();
        if let Some(window) = window() {
            window
                .set_interval_with_callback_and_timeout_and_arguments_0(
                    a.as_ref().unchecked_ref(),
                    duration.as_millis().try_into().unwrap_throw(),
                )
                .unwrap_throw();
        }
    }
}
