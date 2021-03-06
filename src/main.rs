#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate rocket_contrib;

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

/// Returns the stores.
#[get("/store", format = "application/json")]
fn get_store() -> String {
    return crate::cooking_book::store::Store::all_as_json();
}

/// Returns the groups.
#[get("/group", format = "application/json")]
fn get_group() -> String {
    return crate::cooking_book::group::Group::all_as_json();
}

///Returns a list of all ingredients
#[get("/ingredient", format = "application/json")]
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
#[put("/ingredient/<name>", format = "application/json")]
fn put_new_ingredient(name: String) -> String {
    let mut ingredients = persistency::load_ingredients();

    if !ingredients.contains_key(&name) {
        let _ = Ingredient::persist_new_ingredient(&name, &mut ingredients);
    }
    let ingredient = ingredients.get(&name).unwrap();
    let mut shopping_list = persistency::load_shopping_list();
    let _ = shopping_list.add_and_save(&ingredient);

    return shopping_list.to_json();
}

/// Upserts an ingredient.
///
/// #Arguments
///
/// * `name` - The name of the ingredient
/// * `group` - The group of the ingredient
/// * `store` - The store of the ingredient
#[put("/ingredient/<name>/<group>/<store>", format = "application/json")]
fn put_update_ingredient(name: String, group: usize, store: usize) {
    let mut ingredients = persistency::load_ingredients();

    if ingredients.contains_key(&name) {
        let ingredient = ingredients.get_mut(&name).unwrap();
        ingredient.set_group(group);
        ingredient.set_store(store);
    } else {
        let new_ingredient = Ingredient {
            name: name.to_string(),
            group: cooking_book::group::Group::lookup_group_number(group),
            preferred_store: cooking_book::store::Store::lookup_store_number(store),
        };
        ingredients.insert(name, new_ingredient);
    }

    let _ = persistency::write_all_ingredients(&ingredients);
}

/// Removes an ingredient from the shopping list.
/// Returns the updated shopping list.
///
/// #Arguments
///
/// * `name` The name of the ingredient to remove
#[delete("/ingredient/<name>", format = "application/json")]
fn delete_ingredient(name: String) -> String {
    let ingredients = persistency::load_ingredients();

    let mut shopping_list = persistency::load_shopping_list();
    if ingredients.contains_key(&name) {
        let ingredient = ingredients.get(&name).unwrap();
        let _ = shopping_list.remove_and_save(&ingredient);
    }
    return shopping_list.to_json();
}

/// Returns the shopping list.
#[get("/shopping_list", format = "application/json")]
fn get_shopping_list() -> String {
    let shopping_list = persistency::load_shopping_list();
    return shopping_list.to_json();
}

fn main() {
    rocket::ignite()
        .mount(
            "/",
            routes![
                get_store,
                get_group,
                get_ingredient,
                put_new_ingredient,
                put_update_ingredient,
                delete_ingredient,
                get_shopping_list
            ],
        )
        .mount("/", StaticFiles::from("web"))
        .launch();
}