#[macro_use]
extern crate rocket;

#[get("/login")]
fn login() {
    println!("hi");
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![login])
}
