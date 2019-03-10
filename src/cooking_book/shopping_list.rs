use std::collections::HashMap;

use crate::file_access::persistency;
use crate::Ingredient;

#[derive(PartialEq, Eq)]
pub struct ShoppingList {
    pub to_buy: HashMap<Ingredient, u16>,
}

impl ShoppingList {
    pub fn new() -> ShoppingList {
        let to_buy: HashMap<Ingredient, u16> = HashMap::new();
        return ShoppingList { to_buy };
    }

    pub fn add_item(&mut self) {}
    pub fn delete_item(&mut self) {}
    pub fn delete(&mut self) {}

    pub fn print_shopping_list() {
        let shopping_list = persistency::load_shopping_list();
    }
}
