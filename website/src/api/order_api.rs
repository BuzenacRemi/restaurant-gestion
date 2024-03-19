use reqwest::{Client, Error};
use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Order {
    pub id: Option<i32>,
    pub order_id: Option<i32>,
    pub user_id: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Orders(pub Vec<Order>);

pub async fn create_order(user: i32) -> Result<Order, Error> {
    let client = Client::new();

    let order = Order {
        id: None,
        order_id: None,
        user_id: user,

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
