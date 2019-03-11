use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;
use std::process;

use crate::cooking_book::ingredient::Ingredient;
use crate::cooking_book::recipe::Recipe;
use crate::cooking_book::shopping_list::ShoppingList;

fn load_file(file_name: &str) -> String {
    if Path::new(file_name).is_file() {
        return fs::read_to_string(file_name).expect("Something went wrong reading the file");
    }
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(file_name)
        .unwrap();

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Something went wrong reading the file");;
    return contents;
}

pub fn load_shopping_list() -> ShoppingList {
    let mut shopping_list = ShoppingList::new();
    let mut all_ingredients = load_ingredients();

    let content = load_file("shoppingList.csv");
    for line in content.lines() {
        let mut values = line.split(';');
        let name = values.next().unwrap();

        if !all_ingredients.contains_key(name) {
            let new_ingredient = Ingredient::new_by_name(name.to_string());
            write_single_ingredient(&new_ingredient);
            all_ingredients.insert(String::from(name), new_ingredient);
        }

        let amount = match values.next() {
            Some(x) => x,
            None => "",
        };

        let amount = match amount.parse() {
            Ok(x) => x,
            Err(_) => 1,
        };

        shopping_list.add_item(all_ingredients.get(name).unwrap().clone(), amount);
    }

    return shopping_list;
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
        all_ingredients.insert(name.to_string(), Ingredient::new_by_line(line));
    }
    return all_ingredients;
}

pub fn write_all_ingredients(all_ingredients: &Vec<Ingredient>) {
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("ingredients.csv")
        .unwrap();

    for ingredient in all_ingredients {
        write_ingredient(&ingredient, &mut file);
    }
}

pub fn write_single_ingredient(new_ingredient: &Ingredient) {
    let mut file = OpenOptions::new()
        .append(true)
        .open("ingredients.csv")
        .unwrap_or_else(|err| {
            eprintln!("Couldn't open file {}", err);
            process::exit(1);
        });

    write_ingredient(&new_ingredient, &mut file);
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
