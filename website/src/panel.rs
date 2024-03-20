use rocket_dyn_templates::{context, Template};
use crate::api;

#[get("/")]
pub async fn index() -> Template {
    let orders = api::order_api::get_all_orders().await.unwrap();
    let mut orders_with_food = Vec::new();
    for order in orders.0 {
        let ordered_food = api::ordered_food_api::get_ordered_food(order.id_ordered_food.unwrap()).await.unwrap();
        let food = api::food_api::get_food(ordered_food.id_food).await.unwrap();
        orders_with_food.push((order, food));
    }
    Template::render("hbs/panel/index.html.hbs", context! {
        orders: orders_with_food,
    })
}