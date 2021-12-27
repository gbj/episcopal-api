use perseus::t;
use sycamore::prelude::*;

pub fn menu_component<G: GenericNode>(locale: String) -> View<G> {
    // these are necessary to avoid moving `locale` into each of the view attribute closures
    let home = format!("/{locale}/");
    let calendar = format!("/{locale}/calendar");
    let readings = format!("/{locale}/daily-readings");
    let office = format!("/{locale}/daily-office");

    view! {
      nav(id = "main-menu", role = "navigation") {
        // Checkbox
        input(id = "nav-menu-toggle-checkbox", type = "checkbox")
        label(for = "nav-menu-toggle-checkbox", class = "screen-reader-only") {
          (t!("open_menu"))
        }

        // Elements for hamburger lines
        div(class = "hamburger") {
          span { }
          span { }
          span { }
        }

        ul(id = "nav-menu") {
          li {
            h1 {
              a(href = (home)) {
                (t!("common_prayer"))
              }
            }
          }
          li {
            a(href = (calendar)) {
              (t!("calendar"))
            }
          }
          li {
            a(href = (readings)) {
              (t!("daily_readings"))
            }
          }
          li {
            a(href = (office)) {
              (t!("daily_office"))
            }
          }
        }

        div(class = "overlay") { }
      }
    }
}
