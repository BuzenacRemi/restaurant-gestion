use rocket::{get, post, State};
use std::collections::HashMap;
use rocket::http::{Cookie, CookieJar};
use rocket::serde::{Deserialize, Serialize};
use rocket::yansi::Paint;
use rocket_dyn_templates::{context, Template};
use uuid::Uuid;

#[get("/<id>")]
pub fn index(id: i32) -> Template {
    Template::render("hbs/resto/layout", context! {
        id: id.to_string(),
        name: id.to_string(),
        menu: [
            HashMap::from([("name", "Menu 1"), ("price", "42"), ("id", "1")]),
            HashMap::from([("name", "Menu 2"), ("price", "43"), ("id", "2")]),
            HashMap::from([("name", "Menu 3"), ("price", "44"), ("id", "3")]),
        ],
        cart: "Test",
    })
}



