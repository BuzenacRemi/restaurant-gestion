#[macro_use] extern crate rocket;

mod hbs;
mod resto;
mod about;
mod session;
mod cart;
mod objects;
mod statics;
mod cartiframe;
mod api;
mod panel;


use rocket::fs::{FileServer, relative};
use std::collections::HashMap;
use std::time::Duration;
use rocket::http::CookieJar;
use rocket::response::content::RawHtml;
use rocket_dyn_templates::{context, Template};
use std::{env, fs};
use std::path::Path;

#[get("/")]
pub async fn index() -> Template {
    let restos = api::restaurants_api::get_all_restaurants().await.unwrap();
    Template::render("hbs/welcome/layout", context! {
        restos : restos.0,
    })
}
#[launch]
async fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/static", routes![statics::second])
        /*/usr/local/bin/ */
        .mount("/static", FileServer::from(Path::new("assets")))
        .mount("/resto", routes![resto::index])
        .mount("/about", routes![about::index])
        .mount("/cartifram", routes![cartiframe::add_to_cart, cartiframe::view_cart, cartiframe::remove_from_cart])
        .mount("/panel", routes![panel::index])
        .mount("/cart", routes![cart::add_to_cart, cart::view_cart, cart::remove_from_cart, cart::checkout])
        .register("/", catchers![hbs::not_found])
        .attach(Template::fairing ())
}