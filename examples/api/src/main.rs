#[macro_use]
extern crate rocket;

mod calendar;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/calendar", routes![calendar::day])
}
