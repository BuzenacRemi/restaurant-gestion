use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CartItem {
    pub(crate) food_id: String,
    pub(crate) quantity: u32,
}

#[derive(Serialize, Deserialize)]
pub struct Cart {
    items: Vec<CartItem>,
}

impl Cart {
    pub(crate) fn new() -> Cart {
        Cart { items: Vec::new() }
    }

    pub(crate) fn add_item(&mut self ,food_id: String, quantity: u32) {
        match self.items.iter_mut().find(|item| item.food_id == food_id) {
            Some(ref mut item) => item.quantity += quantity,
            None => self.items.push(CartItem { food_id, quantity }),
        }
    }

    pub(crate) fn remove_item(&mut self, food_id: String, quantity: u32) {
        self.items.retain(|item| item.food_id != food_id);
    }
}