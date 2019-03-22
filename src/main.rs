#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate rocket_contrib;

use std::io;
use rocket_contrib::serve::StaticFiles;

mod cooking_book {
    pub mod group;
    pub mod ingredient;
    pub mod recipe;
    pub mod shopping_list;
    pub mod store;
}

mod file_access {
    pub mod persistency;
}

use crate::cooking_book::ingredient::Ingredient;
use crate::file_access::persistency;

///Returns a list of all ingredients
#[get("/ingredient", format="application/json")]
fn get_ingredient() -> String {
    let ingredients = persistency::load_ingredients();
    return Ingredient::all_to_json(&ingredients);
}

/// Adds an ingredient to the shopping list. If the ingredients doesn't exist it will be created.
/// Returns the updated shopping list.
/// 
/// #Arguments
/// 
/// * `name` - The name of the ingredient to add
#[put("/ingredient/<name>", format="application/json")]
fn put_ingredient(name: String) -> String {
    let mut ingredients = persistency::load_ingredients();

    if !ingredients.contains_key(&name) {
        Ingredient::persist_new_ingredient(&name, &mut ingredients);
    }
    let ingredient = ingredients.get(&name).unwrap();
    let mut shopping_list = persistency::load_shopping_list();
    shopping_list.add_and_save(&ingredient);

    return shopping_list.to_json();
}

/// Removes an ingredient from the shopping list.
/// Returns the updated shopping list.
/// 
/// #Arguments
/// 
/// * `name` The name of the ingredient to remove
#[delete("/ingredient/<name>", format="application/json")]
fn delete_ingredient(name: String) -> String {
    let ingredients = persistency::load_ingredients();

    let mut shopping_list = persistency::load_shopping_list();
    if ingredients.contains_key(&name) {
        let ingredient = ingredients.get(&name).unwrap();
        shopping_list.remove_and_save(&ingredient);
    }
    return shopping_list.to_json();
}

/// Returns the shopping list.
#[get("/shopping_list", format="application/json")]
fn get_shopping_list() -> String {
    let shopping_list = persistency::load_shopping_list();
    return shopping_list.to_json();
}

fn main() {
    rocket::ignite()
        .mount("/", routes![get_ingredient, put_ingredient, delete_ingredient, get_shopping_list])
        .mount("/", StaticFiles::from("web"))
        .launch();
}

/// Read from stdin
///
/// # Panics
///
/// The `read_from_stdin` function will panic if it cannot read from stdin
pub fn read_from_stdin() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Couldn't read from stdin");
    return input.trim().to_string();
}