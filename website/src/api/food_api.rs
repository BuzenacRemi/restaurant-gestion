use reqwest::{Client, Error};
use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Food {
    id: Option<i32>,
    category: i32,
    food_name: String,
    food_price: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Foods(pub(crate) Vec<Food>);
pub async fn create_food(category: i32, food_name: String, food_price: i32) -> Result<Food, Error> {
    let client = Client::new();
    let food = Food {
        id: None,
        category,
        food_name,
        food_price,
    };
    let res = client.post("http://api:6969/food")
        .json(&food)
        .send()
        .await?
        .json::<Food>()
        .await?;
    Ok(res)
}

pub async fn get_all_foods() -> Result<Foods, Error> {
    let client = Client::new();
    let res = client.get("http://api:6969/food")
        .send()
        .await?
        .json::<Foods>()
        .await?;
    Ok(res)
}

pub async fn get_food(id: i32) -> Result<Food, Error> {
    let client = Client::new();
    let res = client.get(format!("http://api:6969/food/{}", id))
        .send()
        .await?
        .json::<Food>()
        .await?;
    Ok(res)
}

pub async fn update_food(id: i32, category: i32, food_name: String, food_price: i32) -> Result<Food, Error> {
    let client = Client::new();
    let food = Food {
        id: Some(id),
        category,
        food_name,
        food_price,
    };
    let res = client.put(format!("http://api:6969/food/{}", id))
        .json(&food)
        .send()
        .await?
        .json::<Food>()
        .await?;
    Ok(res)
}

pub async fn delete_food(id: i32) -> Result<(), Error> {
    let client = Client::new();
    client.delete(format!("http://api:6969/food/{}", id))
        .send()
        .await?;
    Ok(())
}

pub async fn get_foods_by_restaurant(id: i32) -> Result<Foods, Error> {
    let client = Client::new();
    let res = client.get(format!("http://api:6969/food/restaurant/{}", id))
        .send()
        .await?
        .json::<Foods>()
        .await?;
    Ok(res)
}
