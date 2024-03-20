use reqwest::{Client, Error};
use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Order {
    pub id: Option<i32>,
    pub id_ordered_food: Option<i32>,
    pub id_client: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Orders(pub Vec<Order>);

pub async fn create_order(user: i32) -> Result<Order, Error> {
    let client = Client::new();
    println!("Creating order for user: {}", user);
    let order = Order {
        id: None,
        id_ordered_food: None,
        id_client: user,

    };
    let res = client.post("http://api:6969/orders")
        .json(&order)
        .send()
        .await?
        .json::<Order>()
        .await?;
    Ok(res)
}

pub async fn get_all_orders() -> Result<Orders, Error> {
    let client = Client::new();
    let res = client.get("http://api:6969/orders")
        .send()
        .await?
        .json::<Orders>()
        .await?;
    Ok(res)
}

pub async fn get_order(id: i32) -> Result<Order, Error> {
    let client = Client::new();
    let res = client.get(format!("http://api:6969/orders/{}", id))
        .send()
        .await?
        .json::<Order>()
        .await?;
    Ok(res)
}

pub async fn delete_order(id: i32) -> Result<(), Error> {
    let client = Client::new();
    client.delete(format!("http://api:6969/orders/{}", id))
        .send()
        .await?;
    Ok(())
}
