use std::collections::HashMap;
use rocket::Data;
use rocket::form::{Form, FromFormField, ValueField};
use rocket::http::{Cookie, CookieJar, SameSite};
use rocket::response::Redirect;
use rocket::serde::Deserialize;
use rocket::serde::json::Json;
use rocket_dyn_templates::{context, Template};
use crate::objects::{Cart, CartItem};

#[post("/add_to_cart", data = "<cart_item>")]
pub async fn add_to_cart(cookies: &CookieJar<'_>, cart_item: Json<CartItem>) -> &'static str {
    let mut cart: Cart = match cookies.get_private("cart") {
        Some(cookie) => serde_json::from_str(cookie.value()).unwrap_or_else(|_| Cart::new()),
        None => Cart::new(),
    };

    cart.add_item(cart_item.food_id.clone(), cart_item.quantity);
    cookies.add_private(Cookie::new("cart", serde_json::to_string(&cart).unwrap()));

    "Item added to cart"
}

#[post("/remove_from_cart", data = "<cart_item>")]
pub async fn remove_from_cart(cookies: &CookieJar<'_>, cart_item: Json<CartItem>) -> &'static str {
    let mut cart: Cart = match cookies.get_private("cart") {
        Some(cookie) => serde_json::from_str(cookie.value()).unwrap_or_else(|_| Cart::new()),
        None => Cart::new(),
    };

    cart.remove_item(cart_item.food_id.clone(), cart_item.quantity);
    cookies.add_private(Cookie::new("cart", serde_json::to_string(&cart).unwrap()));

    "Item removed from cart"
}

#[get("/")]
pub fn view_cart(cookies: &CookieJar<'_>) -> Template {
    /*match cookies.get_private("cart") {
        Some(cookie) => cookie.value().to_string(),
        None => "Votre panier est vide.".to_string(),
    }*/

    Template::render("hbs/cart/layout", context! {
        cart :vec![
            HashMap::from([
                ("food_name", "Menu 1"),
                ("price", "42"),
                ("quantity", "2"),
            ]),
             HashMap::from([
                ("food_name", "Menu 2"),
                ("price", "4.3"),
                ("quantity", "5"),
            ]),
        ]
    })

}