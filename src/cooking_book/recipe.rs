use crate::cooking_book::group::Group;
use crate::cooking_book::ingredient::Ingredient;
use crate::file_access::persistency;
use std::collections::HashMap;
use std::collections::HashSet;

pub struct Recipe {
    pub name: String,
    pub ingredients: HashMap<String, (Ingredient, u16, String)>,
    pub tags: HashSet<String>,
}

impl Recipe {
    pub fn new_by_line(line: &str) -> Result<Recipe, &'static str> {
        let mut values = line.split(';');
        let name = String::from(values.next().unwrap());

        let mut all_ingredients = persistency::load_ingredients();
        let mut ingredients: HashMap<String, (Ingredient, u16, String)> = HashMap::new();
        let mut tags: HashSet<String> = HashSet::new();

        for s in values {
            if s.starts_with('#') {
                tags.insert(s.to_string());
                continue;
            }

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

            ingredients.insert(
                name.to_string(),
                (
                    all_ingredients.get(name).unwrap().clone(),
                    amount,
                    String::from(unit),
                ),
            );
        }

        Ok(Recipe {
            name,
            ingredients,
            tags,
        })
    }

    fn print_recipe(name: &str, recipe: &Recipe) {
        println!("{}", name);
        println!("{:?}", &recipe.tags);

        for (name, (_i, amount, unit)) in &recipe.ingredients {
            println!("\t{}: {} {}", name, amount, unit);
        }
        println!();
    }

    pub fn print_all_recipes_multi_line() {
        let all_recipes = persistency::load_recipes();
        for (name, recipe) in all_recipes {
            crate::cooking_book::recipe::Recipe::print_recipe(name.as_str(), &recipe);
        }
    }

    pub fn print_recipes_by_name() {
        println!("Enter the name of the Recipe");
        let input = crate::read_from_stdin();
        let input = input.as_str();

        let all_recipes = persistency::load_recipes();
        for (name, recipe) in all_recipes.into_iter().filter(|(k, _v)| k.contains(input)) {
            crate::cooking_book::recipe::Recipe::print_recipe(name.as_str(), &recipe);
        }
    }

    pub fn print_recipes_by_used_ingredient() {
        println!("Enter a name of an Ingredient which is used for the Recipe");
        let input = crate::read_from_stdin();
        let input = input.as_str();

        let inputs: Vec<&str> = input.trim().split(',').collect();
        let all_recipes: HashMap<String, Recipe> = persistency::load_recipes();
        for (name, recipe) in all_recipes {
            if recipe
                .ingredients
                .iter()
                .find(|&(name, (_i, _a, _u))| inputs.contains(&name.as_str()))
                .is_none()
            {
                continue;
            }

            crate::cooking_book::recipe::Recipe::print_recipe(name.as_str(), &recipe);
        }
    }

    pub fn print_recipes_by_tag() {
        println!("Enter tags of Recipes to display:");
        let input = crate::read_from_stdin();
        let input = input.as_str();

        let inputs = input.trim().split(',');
        let mut tags: Vec<String> = Vec::new();

        for i in inputs {
            if i.starts_with('#') {
                tags.push(i.to_string());
            }
            else {
                let mut tag = String::with_capacity(i.len() + 1);
                tag.push('#');
                tag.push_str(i);
                tags.push(tag);
            }
        }

        let all_recipes: HashMap<String, Recipe> = persistency::load_recipes();
        for (name, recipe) in all_recipes {
            for tag in &recipe.tags {
                if tags.contains(&tag) {
                    crate::cooking_book::recipe::Recipe::print_recipe(name.as_str(), &recipe);
                    break;
                }
            }
        }
    }
}
