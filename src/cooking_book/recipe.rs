use crate::file_access::persistency;
use crate::cooking_book::group::Group;
use crate::cooking_book::ingredient::Ingredient;
use std::collections::HashMap;

pub struct Recipe {
    pub name: String,
    pub ingredients: Vec<(Ingredient, u16, String)>,
}

impl Recipe {
    pub fn new_by_line(line: &str) -> Result<Recipe, &'static str> {
        let mut values = line.split(';');
        let name = String::from(values.next().unwrap());

        let mut all_ingredients = persistency::load_ingredients();
        let mut ingredients: Vec<(Ingredient, u16, String)> = Vec::new();

        for s in values {
            let mut ingre_amount = s.split(',');
            let name = ingre_amount.next().unwrap();

            let amount = match ingre_amount.next() {
                Some(amount) => amount,
                None => "0",
            };
            let amount = match amount.parse() {
                Ok(num) => num,
                Err(_) => 0,
            };
            let unit = match ingre_amount.next() {
                Some(unit) => unit,
                None => "",
            };

            if !all_ingredients.contains_key(name) {
                let name_str = String::from(name);
                let group = Group::Other;
                let new_ingredient = Ingredient {
                    name: name_str,
                    group,
                };
                persistency::write_ingredient(new_ingredient.clone());
                all_ingredients.insert(String::from(name), new_ingredient.clone());
            }

            ingredients.push((
                all_ingredients.get(name).unwrap().clone(),
                amount,
                String::from(unit),
            ));
        }

        Ok(Recipe { name, ingredients })
    }

    pub fn print_all_recipes_multi_line() {
        let all_recipes = persistency::load_recipes();
        for (name, recipe) in all_recipes {
            println!("{}", name);
            for (ingredient, amount, unit) in recipe.ingredients {
                println!("\t{}: {} {}", ingredient.name, amount, unit);
            }
        }
    }

    pub fn print_recipes_by_name() {
        println!("Enter the name of the recipe");
        let input = crate::read_from_stdin();
        let input = input.as_str();

        let all_recipes = persistency::load_recipes();
        for (name, recipe) in all_recipes.into_iter().filter(|(k, _v)| k.contains(input)) {
            println!("{}", name);
            for (ingredient, amount, unit) in recipe.ingredients {
                println!("\t{}: {} {}", ingredient.name, amount, unit);
            }
        }
    }

    pub fn print_recipes_by_used_ingredient() {
        println!("Enter a name of an Ingredient which is used for the recipe");
        let input = crate::read_from_stdin();
        let input = input.as_str();

        let inputs: Vec<&str> = input.trim().split(',').collect();
        let all_recipes: HashMap<String, Recipe> = persistency::load_recipes();
        for (name, recipe) in all_recipes {
            if recipe
                .ingredients
                .iter()
                .find(|&(i, _a, _u)| inputs.contains(&i.name.as_str()))
                .is_none()
            {
                continue;
            }

            println!("{}", name);
            for (ingredient, amount, unit) in recipe.ingredients {
                println!("\t{}: {} {}", ingredient.name, amount, unit);
            }
        }
    }
}
