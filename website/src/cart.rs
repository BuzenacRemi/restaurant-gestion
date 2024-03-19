use std::collections::HashMap;
use rocket::Data;
use rocket::form::{Contextual, Form, FromFormField, ValueField};
use rocket::http::{Cookie, CookieJar, SameSite};
use rocket::response::Redirect;
use rocket::serde::Deserialize;
use rocket::serde::json::Json;
use rocket::yansi::Paint;
use rocket_dyn_templates::{context, Template};
use serde::Serialize;
use crate::api::food_api::{Food, get_food};
use crate::objects::{Cart, CartItem};
use crate::api::client_api::{Client, create_client};
use crate::api::order_api::create_order;
use crate::api::ordered_food_api::create_ordered_food;

#[derive(Serialize, Deserialize)]
struct CartItemForm {
    food: Food,
    quantity: u32,
}

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

    cart.remove_item(cart_item.food_id.clone());
    cookies.add_private(Cookie::new("cart", serde_json::to_string(&cart).unwrap()));

    "Item removed from cart"
}

#[post("/checkout", data="<client>")]
pub async fn checkout(cookies: &CookieJar<'_>, client: Form<Client>) {
    let client = create_client(client.name.to_string(), client.address.to_string()).await;

    let order = create_order(client.unwrap().id.unwrap()).await.unwrap();
    println!("Order id: {}", order.id.unwrap());

    let mut cart_hash: HashMap<i32, i32> = HashMap::new();
    cookies.get_private("cart").map(|cookie| {
        println!("{}", cookie.value().to_string());
        let mut cart: Cart = serde_json::from_str(cookie.value()).unwrap();
        for item in &cart.items {
            cart_hash.insert(item.food_id as i32, item.quantity as i32);
        }
    });

    for (food_id, quantity) in cart_hash {
        create_ordered_food(order.id.unwrap(), food_id, quantity).await.unwrap();
    }
}

#[get("/")]
pub async fn view_cart(cookies: &CookieJar<'_>) -> Template {
    let cart: Cart = cookies.get_private("cart")
        .and_then(|cookie| serde_json::from_str(cookie.value()).ok())
        .unwrap_or_else(Cart::new);

    let mut context: Vec<CartItemForm> = vec![];
    for item in &cart.items {
        let food = get_food(item.food_id as i32).await.unwrap();
        context.push(CartItemForm {
            food,
            quantity: item.quantity,
        });
        println!("Food id: {}, Quantity: {}", item.food_id, item.quantity);
    }

    Template::render("hbs/cart/layout", context! {
        cart : context,
    })

}