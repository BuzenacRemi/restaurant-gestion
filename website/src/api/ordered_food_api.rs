use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct OrderedFood {
    pub id_order: i32,
    pub id_food: i32,
    pub amount: i32,
}

pub async fn create_ordered_food(order_id: i32, id_food: i32, amount: i32) -> Result<OrderedFood, reqwest::Error> {
    let client = reqwest::Client::new();
    let ordered_food = OrderedFood {
        id_order: order_id,
        id_food: id_food,
        amount: amount,
    };
    let res = client.post("http://api:6969/ordered_food")
        .json(&ordered_food)
        .send()
        .await?
        .json::<OrderedFood>()
        .await?;
    Ok(res)
}

pub async fn get_ordered_food(id: i32) -> Result<OrderedFood, reqwest::Error> {
    let client = reqwest::Client::new();
    let res = client.get(format!("http://api:6969/ordered_food/{}", id))
        .send()
        .await?
        .json::<OrderedFood>()
        .await?;
    Ok(res)
}

pub async fn get_all_ordered_foods() -> Result<Vec<OrderedFood>, reqwest::Error> {
    let client = reqwest::Client::new();
    let res = client.get("http://api:6969/ordered_food")
        .send()
        .await?
        .json::<Vec<OrderedFood>>()
        .await?;
    Ok(res)
}

