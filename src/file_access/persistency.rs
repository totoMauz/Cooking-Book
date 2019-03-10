use std::collections::HashMap;
use std::fs;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::process;

use crate::cooking_book::ingredient::Ingredient;
use crate::cooking_book::recipe::Recipe;
use crate::cooking_book::shopping_list::ShoppingList;

fn load_file(file_name: &str) -> String {
    return fs::read_to_string(file_name).expect("Something went wrong reading the file");
}

pub fn load_shopping_list() -> ShoppingList {
    let content = load_file("shoppingList.csv");

    return ShoppingList::new();
}

pub fn load_recipes() -> HashMap<String, Recipe> {
    let content = load_file("recipes.csv");
    let mut all_recipes: HashMap<String, Recipe> = HashMap::new();

    for line in content.lines() {
        if line.starts_with("#") {
            continue;
        }

        let name = line.split(';').next().unwrap();

        all_recipes.insert(
            String::from(name),
            Recipe::new_by_line(line).unwrap_or_else(|err| {
                eprintln!("Problem restoring Recipe: {}", err);
                process::exit(1);
            }),
        );
    }
    return all_recipes;
}

pub fn load_ingredients() -> HashMap<String, Ingredient> {
    let content = load_file("ingredients.csv");
    let mut all_ingredients: HashMap<String, Ingredient> = HashMap::new();

    for line in content.lines() {
        if line.starts_with("#") {
            continue;
        }

        let name = line.split(';').next().unwrap();

        all_ingredients.insert(
            String::from(name),
            Ingredient::new_by_line(line).unwrap_or_else(|err| {
                eprintln!("Problem restoring Ingredient: {}", err);
                process::exit(1);
            }),
        );
    }
    return all_ingredients;
}

pub fn write_all_ingredients(all_ingredients: Vec<Ingredient>) {
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("ingredients.csv")
        .unwrap();

    for ingredient in all_ingredients {
        if let Err(e) = writeln!(file, "{};{}", ingredient.name, ingredient.group as i8) {
            eprintln!("Couldn't write to file: {}", e);
        }
    }
}

pub fn write_ingredient(new_ingredient: Ingredient) {
    let mut file = OpenOptions::new()
        .append(true)
        .open("ingredients.csv")
        .unwrap_or_else(|err| {
            eprintln!("Couldn't open file {}", err);
            process::exit(1);
        });
    if let Err(e) = writeln!(
        file,
        "{};{}",
        new_ingredient.name, new_ingredient.group as i8
    ) {
        eprintln!("Couldn't write to file: {}", e);
    }
}
