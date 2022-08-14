pub mod controller;

use rocket::routes;
use rocket::launch;
use controller::index::index;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
