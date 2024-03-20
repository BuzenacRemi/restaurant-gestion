use rocket::serde::json::Json;
use serde_json::json;
use crate::api;

pub async fn get_everything() -> Json<String> {
    //Get all datas from database and return it in JSON format
    //Start by restaurants
    let orders = api::order_api::get_all_orders().await.unwrap();
    let clients = api::client_api::get_all_clients().await.unwrap();
    let ordered_foods = api::ordered_food_api::get_all_ordered_foods().await.unwrap();
    let foods = api::food_api::get_all_foods().await.unwrap();

    let everything = json!(format!("{{\"orders\": {:?}, \"clients\": {:?}, \"ordered_foods\": {:?}, \"foods\": {:?}}}", orders, clients, ordered_foods, foods)).into();
    everything

}