use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;

use crate::cooking_book::ingredient::Ingredient;
use crate::cooking_book::recipe::Recipe;
use crate::cooking_book::shopping_list::ShoppingList;

fn load_file(file_name: &str) -> Option<String> {
    if Path::new(file_name).is_file() {
        return match fs::read_to_string(file_name) {
            Ok(c) => Some(c),
            Err(_) => None,
        };
    }
    let file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(file_name);

    if file.is_err() {
        return None;
    }

    let mut contents = String::new();
    return match file.unwrap().read_to_string(&mut contents) {
        Ok(_) => Some(contents),
        Err(_) => None,
    };
}

pub fn load_shopping_list() -> ShoppingList {
    let mut shopping_list = ShoppingList::new();

    let content = load_file("shoppingList.csv");
    if content.is_none() {
        return shopping_list;
    }

    let mut all_ingredients = load_ingredients();

    for line in content.unwrap().lines() {
        let mut values = line.split(';');
        let name = values.next().unwrap();

        if !all_ingredients.contains_key(name) {
            Ingredient::persist_new_ingredient(name.to_string(), &mut all_ingredients);
        }

        let amount = match values.next() {
            Some(x) => x,
            None => "",
        };

        let amount = match amount.parse::<u16>() {
            Ok(x) => x,
            Err(_) => 1,
        };

        shopping_list.add_item(all_ingredients.get(name).unwrap().clone(), amount);
    }

    return shopping_list;
}

pub fn write_shopping_list(shopping_list: &ShoppingList) {
    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("shoppingList.csv");

    if file.is_ok() {
        let mut file = file.unwrap();
        for (ingredient, amount) in &shopping_list.to_buy {
            if let Err(e) = writeln!(file, "{};{}", ingredient.name, amount) {
                eprintln!("Couldn't write to file: {}", e);
            }
        }
    }
}

pub fn load_recipes() -> HashMap<String, Recipe> {
    let mut all_recipes: HashMap<String, Recipe> = HashMap::new();

    let content = load_file("recipes.csv");
    if content.is_none() {
        return all_recipes;
    }

    for line in content.unwrap().lines() {
        if line.starts_with("#") {
            continue;
        }

        let name = line.split(';').next().unwrap();

        all_recipes.insert(
            String::from(name),
            Recipe::new_by_line(line)
        );
    }
    return all_recipes;
}

pub fn load_ingredients() -> HashMap<String, Ingredient> {
    let mut all_ingredients: HashMap<String, Ingredient> = HashMap::new();

    let content = load_file("ingredients.csv");
    if content.is_none() {
        return all_ingredients;
    }

    for line in content.unwrap().lines() {
        if line.starts_with("#") {
            continue;
        }

        let name = line.split(';').next().unwrap();
        all_ingredients.insert(name.to_string(), Ingredient::new_by_line(line));
    }
    return all_ingredients;
}

pub fn write_all_ingredients(all_ingredients: &Vec<Ingredient>) {
    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("ingredients.csv");

    if file.is_ok() {
        let mut file = file.unwrap();
        for ingredient in all_ingredients {
            write_ingredient(&ingredient, &mut file);
        }
    }
}

pub fn write_single_ingredient(new_ingredient: &Ingredient) {
    let file = OpenOptions::new().append(true).open("ingredients.csv");

    if file.is_ok() {
        write_ingredient(&new_ingredient, &mut file.unwrap());
    }
}

fn write_ingredient(ingredient: &Ingredient, file: &mut File) {
    if let Err(e) = writeln!(
        file,
        "{};{};{}",
        ingredient.name, ingredient.group as i8, ingredient.preferred_store as i8
    ) {
        eprintln!("Couldn't write to file: {}", e);
    }
}
