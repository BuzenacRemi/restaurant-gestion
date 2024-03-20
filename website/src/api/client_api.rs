use reqwest::Error;
use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug,FromForm)]
pub struct Client {
    pub id: Option<i32>,
    pub name: String,
    pub address: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Clients(pub(crate) Vec<Client>);

pub async fn create_client(name: String, address: String) -> Result<Client, Error> {
    let client_user = Client {
        id: None,
        name,
        address,
    };
    let client = reqwest::Client::new();
    let res = client.post("http://api:6969/clients")
        .json(&client_user)
        .send()
        .await?
        .json::<Client>()
        .await?;
    Ok(res)
}

pub async fn get_client(id: i32) -> Result<Client, Error> {
    let client = reqwest::Client::new();
    let res = client.get(format!("http://api:6969/clients/{}", id))
        .send()
        .await?
        .json::<Client>()
        .await?;
    Ok(res)
}

pub async fn get_client_by_name(name: String) -> Result<Client, Error> {
    let client = reqwest::Client::new();
    let res = client.get(format!("http://api:6969/clients/{}", name))
        .send()
        .await?
        .json::<Client>()
        .await?;
    Ok(res)
}

pub async fn get_all_clients() -> Result<Clients, Error> {
    let client = reqwest::Client::new();
    let res = client.get("http://api:6969/clients")
        .send()
        .await?
        .json::<Clients>()
        .await?;
    Ok(res)
}


