use crate::utils::input::value;
use sycamore::prelude::*;
use web_sys::Event;

pub struct Toggle {
    name: String,
    off_label: String,
    on_label: String,
    state: Signal<bool>,
    pub toggled: ReadSignal<bool>,
}

impl Toggle {
    pub fn new(name: String, off_label: String, on_label: String, starts_toggled: bool) -> Self {
        let state = Signal::new(starts_toggled);
        let toggled = state.handle();
        Self {
            name,
            off_label,
            on_label,
            state,
            toggled,
        }
    }

    pub fn toggled(&self) -> ReadSignal<bool> {
        self.toggled.clone()
    }

    pub fn view<G: GenericNode<EventType = Event>>(self) -> View<G> {
        let check_1 = self.state.clone();
        let check_2 = self.state.clone();
        let state = self.state;
        let name = self.name;
        let name_2 = name.clone();
        let id_off = format!("{}-off", name);
        let id_off_2 = id_off.clone();
        let id_on = format!("{}-on", name);
        let id_on_2 = id_on.clone();
        let off_label = self.off_label;
        let on_label = self.on_label;

        view! {
            fieldset(class = "toggle") {
                input(
                    type = "radio",
                    id = (id_off),
                    name = (name),
                    value = "off",
                    checked=!(*check_1.get()),
                    on:change=cloned!((state) => move |ev: Event| if value(ev) == "off" { state.set(false) })
                )
                label(for = id_off_2) {
                    (off_label)
                }
                input(
                    type = "radio",
                    id = (id_on),
                    name = (name_2),
                    value = "on",
                    checked=*check_2.get(),
                    on:change=cloned!((state) => move |ev: Event| if value(ev) == "on" { state.set(true) })
                )
                label(for = id_on_2) {
                    (on_label)
                }
            }
        }
    }
}
