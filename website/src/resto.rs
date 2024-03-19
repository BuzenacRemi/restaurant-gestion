use rocket::{get, post, State};
use std::collections::HashMap;
use rocket::http::{Cookie, CookieJar};
use rocket::serde::{Deserialize, Serialize};
use rocket::yansi::Paint;
use rocket_dyn_templates::{context, Template};
use uuid::Uuid;
use crate::api::food_api::{get_all_foods, get_foods_by_restaurant};
use crate::api::restaurants_api::get_restaurant;

#[get("/<id>")]
pub async fn index(id: i32) -> Template {
    let foods = get_foods_by_restaurant(id).await.unwrap();
    let restaurant_name = get_restaurant(id).await.unwrap().name;


    Template::render("hbs/resto/layout", context! {
        id: id.to_string(),
        name: restaurant_name,
        menu: foods.0,
    })
}



