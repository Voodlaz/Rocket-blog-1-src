#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

#[get("/")]
pub fn homepage() -> String {
    format!("homepage")
}

fn main() {
    rocket::ignite()
    .mount("/", routes![homepage])
    .launch();
}
