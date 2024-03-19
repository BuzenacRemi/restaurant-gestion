#[macro_use] extern crate rocket;

mod hbs;
mod resto;
mod about;
mod session;
mod cart;
mod objects;
mod statics;
mod cartiframe;


use rocket::fs::{FileServer, relative};
use std::collections::HashMap;
use std::time::Duration;
use rocket::http::CookieJar;
use rocket::response::content::RawHtml;
use rocket_dyn_templates::{context, Template};
use std::{env, fs};
use std::path::Path;

#[get("/")]
pub fn index() -> Template {
    Template::render("hbs/welcome/layout", context! {
        restos : [HashMap::from([
            ("name", "Resto 1"),
            ("id", "42"),
        ]), HashMap::from([
            ("name", "Resto 2"),
            ("id", "43"),
        ]), HashMap::from([
            ("name", "Resto 3"),
            ("id", "44"),
        ],)]
    })
}
#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/static", routes![statics::second])
        /*/usr/local/bin/ */
        .mount("/static", FileServer::from(Path::new("assets")))
        .mount("/resto", routes![resto::index])
        .mount("/about", routes![about::index])
        .mount("/cart", routes![cart::add_to_cart, cart::view_cart, cart::remove_from_cart])
        .mount("/cartifram", routes![cartiframe::add_to_cart, cartiframe::view_cart, cartiframe::remove_from_cart])
        .register("/", catchers![hbs::not_found])
        .attach(Template::fairing())
}