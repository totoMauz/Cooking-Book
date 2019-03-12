use std::io;

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

use crate::cooking_book::group::Group;
use crate::cooking_book::ingredient::Ingredient;
use crate::cooking_book::recipe::Recipe;
use crate::cooking_book::shopping_list::ShoppingList;
use crate::cooking_book::store::Store;

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

fn main() {
    loop {
        print_menu();
        let main_menu = read_from_stdin();

        match main_menu.as_str() {
            "0" => break,
            "1" => loop {
                print_menu_ingredients();
                let sub_menu = read_from_stdin();

                match sub_menu.as_str() {
                    "0" => break,
                    "1" => Ingredient::print_all_ingredients_multi_line(),
                    "2" => Ingredient::save_new_ingredient(),
                    "3" => Ingredient::delete_ingredient(),
                    &_ => continue,
                }
            },
            "2" => loop {
                print_menu_recipes();
                let sub_menu = read_from_stdin();

                match sub_menu.as_str() {
                    "0" => break,
                    "1" => Recipe::print_all_recipes_multi_line(),
                    "2" => Recipe::print_recipes_by_name(),
                    "3" => Recipe::print_recipes_by_used_ingredient(),
                    "4" => Recipe::print_recipes_by_tag(),
                    &_ => continue,
                }
            },
            "3" => loop {
                print_menu_shopping_list();
                let sub_menu = read_from_stdin();

                match sub_menu.as_str() {
                    "0" => break,
                    "1" => ShoppingList::print_shopping_list(),
                    "2" => ShoppingList::add_ingredient(),
                    "3" => ShoppingList::remove_ingredient(),
                    &_ => continue,
                }
            },
            "4" => Group::print_all_groups_multi_line(),
            "5" => Store::print_all_stores_multi_line(),
            &_ => eprintln!("Unrecognized input {}", main_menu),
        }
    }
}

fn print_menu() {
    println!("-------------------");
    println!("0: Exit");
    println!("1: Ingredients");
    println!("2: Recipes");
    println!("3: Shopping List");
    println!("4: Show all Groups");
    println!("5: Show all Stores");
    println!("-------------------");
}

fn print_menu_ingredients() {
    println!("-------------------");
    println!("0: Back");
    println!("1: Show all Ingredients");
    println!("2: Add Ingredient");
    println!("3: Remove Ingredient");
    println!("-------------------");
}

fn print_menu_recipes() {
    println!("-------------------");
    println!("0: Back");
    println!("1: Show all Recipes");
    println!("2: Show Recipes by Name");
    println!("3: Show Recipes by Ingredient");
    println!("4: Show Recipes by Tag");
    println!("-------------------");
}

fn print_menu_shopping_list() {
    println!("-------------------");
    println!("0: Back");
    println!("1: Show Shopping List");
    println!("2: Add Item");
    println!("3: Remove Item");
    println!("-------------------");
}
