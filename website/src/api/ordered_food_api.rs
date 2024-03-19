use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct OrderedFood {
    pub order_id: i32,
    pub food_id: i32,
    pub quantity: i32,
}

pub async fn create_ordered_food(order_id: i32, food_id: i32, quantity: i32) -> Result<OrderedFood, reqwest::Error> {
    let client = reqwest::Client::new();
    let ordered_food = OrderedFood {
        order_id,
        food_id,
        quantity,
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

