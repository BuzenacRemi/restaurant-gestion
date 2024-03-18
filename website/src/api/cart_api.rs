use reqwest::{Client, Error};
use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct OrderedFood {
    pub food_id: i32,
    pub quantity: i32,
}

pub async fn add_to_cart(food_id: i32, quantity: i32) -> Result<(), Error> {
    let client = Client::new();
    let ordered_food = OrderedFood {
        food_id,
        quantity,
    };
    client.post("http://api:6969/cart")
        .json(&ordered_food)
        .send()
        .await?;
    Ok(())
}

pub async fn view_cart() -> Result<Vec<OrderedFood>, Error> {
    let client = Client::new();
    let res = client.get("http://api:6969/cart")
        .send()
        .await?
        .json::<Vec<OrderedFood>>()
        .await?;
    Ok(res)
}

pub async fn remove_from_cart(food_id: i32) -> Result<(), Error> {
    let client = Client::new();
    client.delete(format!("http://api:6969/cart/{}", food_id))
        .send()
        .await?;
    Ok(())
}

pub async fn clear_cart() -> Result<(), Error> {
    let client = Client::new();
    client.delete("http://api:6969/cart")
        .send()
        .await?;
    Ok(())
}