use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CartItem {
    pub(crate) food_id: u32,
    pub(crate) quantity: u32,
}

impl CartItem {
    pub(crate) fn new(food_id: u32, quantity: u32) -> CartItem {
        CartItem { food_id, quantity }
    }

    pub(crate) fn display(&self) {
        println!("Food id: {}, Quantity: {}", self.food_id, self.quantity);
    }

    pub(crate) fn clone(&self) -> CartItem {
        CartItem { food_id: self.food_id, quantity: self.quantity }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Cart {
    pub(crate) items: Vec<CartItem>,
}

impl Cart {
    pub(crate) fn new() -> Cart {
        Cart { items: Vec::new() }
    }

    pub(crate) fn add_item(&mut self ,food_id: u32, quantity: u32) {
        match self.items.iter_mut().find(|item| item.food_id == food_id) {
            Some(ref mut item) => item.quantity += quantity,
            None => self.items.push(CartItem { food_id, quantity }),
        }
    }

    pub(crate) fn remove_item(&mut self, food_id: u32) {
        self.items.retain(|item| item.food_id != food_id);
    }

    pub(crate) fn display(&self) {
        for item in &self.items {
            item.display();
        }
    }

    pub(crate) fn clear(&mut self) {
        self.items.clear();
    }
}