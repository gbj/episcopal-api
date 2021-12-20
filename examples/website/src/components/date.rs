use calendar::Date;
use sycamore::prelude::*;
use web_sys::Event;

pub fn date_picker<G: GenericNode<EventType = Event>>(
    id: &'static str,
    label: String,
    initial_date: Date,
) -> (ReadSignal<Option<Date>>, View<G>) {
    let date_input_value = Signal::new(initial_date.to_string());
    let date = create_memo({
        let date_input_value = date_input_value.clone();
        move || Date::parse_from_str(&*date_input_value.get(), "%Y-%m-%d").ok()
    });

    let view = view! {
      fieldset(class = "centered stacked") {
          label(for = (id)) {
              (label)
          }
          input(
            type="date",
            id = (id),
            bind:value=date_input_value
          )
      }
    };

    (date, view)
}
