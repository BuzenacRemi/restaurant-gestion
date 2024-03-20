use reqwest::{Client, Error};
use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Restaurant {
    id: Option<i32>,
    pub(crate) name: String,
    zip_code: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Restaurants(pub(crate) Vec<Restaurant>);

pub async fn create_restaurant(name: String, zip_code: String) -> Result<Restaurant, Error> {
    let client = Client::new();
    let restaurant = Restaurant {
        id: None,
        name,
        zip_code,
    };
    let res = client.post("http://api:6969/restaurants")
        .json(&restaurant)
        .send()
        .await?
        .json::<Restaurant>()
        .await?;
    Ok(res)
}

pub async fn get_all_restaurants() -> Result<Restaurants, Error> {
    let client = Client::new();
    let res = client.get("http://api:6969/restaurants")
        .send()
        .await?
        .json::<Restaurants>()
        .await?;
    Ok(res)
}

pub async fn get_restaurant(id: i32) -> Result<Restaurant, Error> {
    let client = Client::new();
    let res = client.get(format!("http://api:6969/restaurants/{}", id))
        .send()
        .await?
        .json::<Restaurant>()
        .await?;
    Ok(res)
}

pub async fn update_restaurant(id: i32, name: String, zip_code: String) -> Result<Restaurant, Error> {
    let client = Client::new();
    let restaurant = Restaurant {
        id: Some(id),
        name,
        zip_code,
    };
    let res = client.put(format!("http://api:6969/restaurants/{}", id))
        .json(&restaurant)
        .send()
        .await?
        .json::<Restaurant>()
        .await?;
    Ok(res)
}

pub async fn delete_restaurant(id: i32) -> Result<(), Error> {
    let client = Client::new();
    client.delete(format!("http://api:6969/restaurants/{}", id))
        .send()
        .await?;
    Ok(())
}