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

    let content = load_file("persistency/shoppingList.csv");
    if content.is_none() {
        return shopping_list;
    }

    let mut all_ingredients = load_ingredients();

    for line in content.unwrap().lines() {
        let mut values = line.split(';');
        let name = values.next().unwrap().to_string();

        if !all_ingredients.contains_key(&name) {
            Ingredient::persist_new_ingredient(&name, &mut all_ingredients).unwrap_or_else(|e| eprintln!("{}", e));
        }

        let amount = match values.next() {
            Some(x) => x,
            None => "",
        };

        let amount = match amount.parse::<u16>() {
            Ok(x) => x,
            Err(_) => 1,
        };

        shopping_list.add_item(all_ingredients.get(&name).unwrap().clone(), amount);
    }

    return shopping_list;
}

pub fn write_shopping_list(shopping_list: &ShoppingList) -> Result<(), String> {
    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("persistency/shoppingList.csv");

    if file.is_ok() {
        let mut file = file.unwrap();
        for (ingredient, amount) in &shopping_list.to_buy {
            if let Err(e) = writeln!(file, "{};{}", ingredient.name, amount) {
                return Err(format!("Couldn't write to file: {}", e));
            }
        }
    }
    return Ok(());
}

pub fn load_recipes() -> HashMap<String, Recipe> {
    let mut all_recipes: HashMap<String, Recipe> = HashMap::new();

    let content = load_file("persistency/recipes.csv");
    if content.is_none() {
        return all_recipes;
    }

    for line in content.unwrap().lines() {
        if line.starts_with("#") {
            continue;
        }

        let name = line.split(';').next().unwrap();

        all_recipes.insert(String::from(name), Recipe::new_by_line(line));
    }
    return all_recipes;
}

pub fn load_ingredients() -> HashMap<String, Ingredient> {
    let mut all_ingredients: HashMap<String, Ingredient> = HashMap::new();

    let content = load_file("persistency/ingredients.csv");
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

pub fn write_all_ingredients(all_ingredients: &Vec<Ingredient>) -> Result<(), String> {
    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("persistency/ingredients.csv");

    if file.is_ok() {
        let mut file = file.unwrap();
        for ingredient in all_ingredients {
            write_ingredient(&ingredient, &mut file).unwrap_or_else(|e| eprintln!("{}", e));
        }
        return Ok(());
    }
    return Err(format!("Couldn't write to file: {}", file.unwrap_err()));
}

pub fn write_single_ingredient(new_ingredient: &Ingredient) -> Result<(), String> {
    let file = OpenOptions::new().append(true).open("ingredients.csv");

    if file.is_ok() {
        return write_ingredient(&new_ingredient, &mut file.unwrap());
    }
    return Err(format!("Couldn't open file: {}", file.unwrap_err()));
}

fn write_ingredient(ingredient: &Ingredient, file: &mut File) -> Result<(), String> {
    if let Err(e) = writeln!(
        file,
        "{};{};{}",
        ingredient.name, ingredient.group as i8, ingredient.preferred_store as i8
    ) {
        return Err(format!("Couldn't write to file: {}", e));
    }
    return Ok(());
}
