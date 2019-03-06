use crate::cooking_book::group::Group;
use crate::cooking_book::ingredient::Ingredient;
use crate::file_access::persistency;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(PartialEq, Eq)]
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
            Recipe::print_recipe(name.as_str(), &recipe);
        }
    }

    fn get_recipes_by_name<'a>(
        recipes: &'a HashMap<String, Recipe>,
        name: &str,
    ) -> Vec<&'a Recipe> {
        let mut recipes_by_name: Vec<&'a Recipe> = Vec::with_capacity(recipes.len());

        for (_n, recipe) in recipes.iter().filter(|(k, _v)| k.contains(name)) {
            recipes_by_name.push(recipe);
        }

        return recipes_by_name;
    }

    fn get_recipes_by_ingredients<'a>(
        recipes: &'a HashMap<String, Recipe>,
        ingredient_names: Vec<&str>,
    ) -> Vec<&'a Recipe> {
        let mut recipes_by_ingredient: Vec<&Recipe> = Vec::with_capacity(recipes.len());

        for (_n, recipe) in recipes {
            if recipe
                .ingredients
                .iter()
                .find(|&(name, (_i, _a, _u))| ingredient_names.contains(&name.as_str()))
                .is_none()
            {
                continue;
            }

            recipes_by_ingredient.push(recipe);
        }

        return recipes_by_ingredient;
    }

    fn get_recipes_by_tags<'a>(
        recipes: &'a HashMap<String, Recipe>,
        tags: &Vec<String>,
    ) -> Vec<&'a Recipe> {
        let mut recipes_by_tag: Vec<&Recipe> = Vec::with_capacity(recipes.len());
        for (_n, recipe) in recipes {
            for tag in &recipe.tags {
                if tags.contains(&tag) {
                    recipes_by_tag.push(recipe);
                    break;
                }
            }
        }
        return recipes_by_tag;
    }

    pub fn print_recipes_by_name() {
        println!("Enter the name of the Recipe");
        let input = crate::read_from_stdin();
        let all_recipes = persistency::load_recipes();

        for recipe in Recipe::get_recipes_by_name(&all_recipes, input.as_str()) {
            Recipe::print_recipe(&recipe.name, &recipe);
        }
    }

    pub fn print_recipes_by_used_ingredient() {
        println!("Enter a name of an Ingredient which is used for the Recipe");
        let input = crate::read_from_stdin();
        let input = input.as_str();
        let all_recipes = persistency::load_recipes();

        let inputs: Vec<&str> = input.trim().split(',').collect();
        for recipe in Recipe::get_recipes_by_ingredients(&all_recipes, inputs) {
            Recipe::print_recipe(&recipe.name, &recipe);
        }
    }

    fn unify_tags(input: &str) -> Vec<String> {
        let mut tags: Vec<String> = Vec::new();

        let inputs = input.trim().split(',');
        for i in inputs {
            if i.starts_with('#') {
                tags.push(i.to_string());
            } else {
                let mut tag = String::with_capacity(i.len() + 1);
                tag.push('#');
                tag.push_str(i);
                tags.push(tag);
            }
        }
        return tags;
    }

    pub fn print_recipes_by_tag() {
        println!("Enter tags of Recipes to display:");
        let input = crate::read_from_stdin();
        let tags = Recipe::unify_tags(input.as_str());
        let all_recipes = persistency::load_recipes();

        for recipe in Recipe::get_recipes_by_tags(&all_recipes, &tags) {
            Recipe::print_recipe(&recipe.name, &recipe);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Recipe;
    use crate::Ingredient;
    use std::collections::HashMap;
    use std::collections::HashSet;

    #[test]
    fn test_unify_tag() {
        let tags = "a,#b";
        let tags = Recipe::unify_tags(tags);

        let tag_a = "#a".to_string();
        assert!(tags.contains(&tag_a));

        let tag_b = "#b".to_string();
        assert!(tags.contains(&tag_b));
    }

    fn get_mocks() -> HashMap<String, Recipe> {
                let mut recipes: HashMap<String, Recipe> = HashMap::with_capacity(2);

        let name1 = "R1".to_string();
        let ingredients1: HashMap<String, (Ingredient, u16, String)> = HashMap::new();
        let mut tags1: HashSet<String> = HashSet::new();
        tags1.insert("1".to_string());
        tags1.insert("3".to_string());

        let r1 = Recipe {
            name: name1,
            ingredients: ingredients1,
            tags: tags1,
        };
        recipes.insert("R1".to_string(), r1);

        let name2 = "R2".to_string();
        let ingredients2: HashMap<String, (Ingredient, u16, String)> = HashMap::new();
        let mut tags2: HashSet<String> = HashSet::new();
        tags2.insert("2".to_string());
        tags2.insert("3".to_string());

        let r2 = Recipe {
            name: name2,
            ingredients: ingredients2,
            tags: tags2,
        };

        recipes.insert("R2".to_string(), r2);

        return recipes;
    }

    #[test]
    fn test_by_name() {
        let recipes = self::get_mocks();

        let recipes_r = Recipe::get_recipes_by_name(&recipes, "R");
        assert!(recipes_r.contains(&recipes.get("R1").unwrap()));
        assert!(recipes_r.contains(&recipes.get("R2").unwrap()));

        let recipes_1 = Recipe::get_recipes_by_name(&recipes, "1");
        assert!(recipes_1.contains(&recipes.get("R1").unwrap()));
        assert!(!recipes_1.contains(&recipes.get("R2").unwrap()));

        let recipes_2 = Recipe::get_recipes_by_name(&recipes, "2");
        assert!(!recipes_2.contains(&recipes.get("R1").unwrap()));
        assert!(recipes_2.contains(&recipes.get("R2").unwrap()));
    }

    #[test]
    fn test_by_tag() {
        let recipes = self::get_mocks();
        let mut tags: Vec<String> = Vec::with_capacity(2);
        let tag = "1".to_string();
        tags.push(tag);

        let recipes_r = Recipe::get_recipes_by_tags(&recipes, &tags);
        assert!(recipes_r.contains(&recipes.get("R1").unwrap()));
        assert!(!recipes_r.contains(&recipes.get("R2").unwrap()));

        tags.clear();
        let tag = "2".to_string();
        tags.push(tag);
        let recipes_1 = Recipe::get_recipes_by_tags(&recipes, &tags);
        assert!(!recipes_1.contains(&recipes.get("R1").unwrap()));
        assert!(recipes_1.contains(&recipes.get("R2").unwrap()));

        tags.clear();
        let tag = "3".to_string();
        tags.push(tag);
        let recipes_2 = Recipe::get_recipes_by_tags(&recipes, &tags);
        assert!(recipes_2.contains(&recipes.get("R1").unwrap()));
        assert!(recipes_2.contains(&recipes.get("R2").unwrap()));
    }
}
