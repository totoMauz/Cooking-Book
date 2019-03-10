use std::cmp::Ordering;
use std::fmt;

use crate::cooking_book::group::Group;
use crate::cooking_book::store::Store;
use crate::file_access::persistency;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Ingredient {
    pub name: String,
    pub group: Group,
    pub preferred_store: Store,
}

impl Ingredient {
    pub fn new_by_line(line: &str) -> Result<Ingredient, &'static str> {
        let mut values = line.split(';');

        let name = String::from(values.next().unwrap());
        let group = match values.next() {
            Some(group) => group.trim(),
            None => "<empty>",
        };
        let group: Group = match group.parse() {
            Ok(num) => Group::lookup_group_number(num),
            Err(_) => Group::Other,
        };

        let store = match values.next() {
            Some(store) => store.trim(),
            None => "<empty>",
        };
        let store: Store = match store.parse() {
            Ok(num) => Store::lookup_store_number(num),
            Err(_) => Store::Any,
        };

        Ok(Ingredient {
            name,
            group,
            preferred_store: store,
        })
    }

    pub fn to_json(&self) -> String{
        let mut json :String = String::new();
        json.push('{');

        json.push_str("\"name\": \"");
        json.push_str(&self.name);
        json.push_str("\", ");
        
        json.push_str("\"group\": \"");
        json.push_str(&format!("{}", &self.group));
        json.push_str("\", ");
        
        json.push_str("\"store\": \"");
        json.push_str(&format!("{:?}", &self.preferred_store));
        json.push_str("\"");

        json.push('}');

        return json;
    }

    pub fn save_new_ingredient() {
        println!("Enter a name and the group like this Name;0");
        println!("Possible groups are: ");
        Group::print_all_groups_single_line();

        let input = crate::read_from_stdin();
        let new_ingredient = Ingredient::new_by_line(input.as_str()).unwrap();
        persistency::write_ingredient(new_ingredient);
    }

    pub fn delete_ingredient() {
        println!("Enter a name of an Ingredient to delete");
        let input = crate::read_from_stdin();

        let mut all_ingredients = Ingredient::get_all_ingredients();
        let found_ingredient = all_ingredients.iter().position(|x| x.name == input);

        match found_ingredient {
            Some(position) => {
                all_ingredients.remove(position);
                persistency::write_all_ingredients(all_ingredients);
            }
            None => eprintln!("Couldn't find Ingredient {}", input),
        }
    }

    pub fn get_all_ingredients() -> Vec<Ingredient> {
        let all_ingredients = persistency::load_ingredients();
        let mut vec_ingredients: Vec<Ingredient> = Vec::new();

        for (_key, value) in all_ingredients.iter() {
            vec_ingredients.push(value.clone());
        }
        vec_ingredients.sort();

        return vec_ingredients;
    }

    pub fn print_all_ingredients_multi_line() {
        let all_ingredients = Ingredient::get_all_ingredients();
        for ingredient in all_ingredients {
            println!("{}", ingredient);
        }
    }
}

impl PartialOrd for Ingredient {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Ingredient {
    fn cmp(&self, other: &Self) -> Ordering {
        let order_group = self.group.cmp(&other.group);
        if order_group == Ordering::Equal {
            return self.name.cmp(&other.name);
        }
        return order_group;
    }
}

impl fmt::Display for Ingredient {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} [{}]", &self.name, &self.group)
    }
}

#[cfg(test)]
mod tests {
    use super::Ingredient;
    use crate::cooking_book::group::Group;
    use crate::cooking_book::store::Store;
    use std::cmp::Ordering;

    #[test]
    fn test_to_json() {
        let ingredient = Ingredient{name: "Gurke".to_string(), group: Group::Vegetable, preferred_store: Store::Lidl};
        assert_eq!(ingredient.to_json(), "{\"name\": \"Gurke\", \"group\": \"Gemüse\", \"store\": \"Lidl\"}");
    }

    #[test]
    fn test_new_by_line_empty_1() {
        let ingredient = Ingredient::new_by_line("").unwrap();
        assert_eq!(ingredient.name, "");
        assert_eq!(ingredient.group, Group::Other);
    }

    #[test]
    fn test_new_by_line_empty_2() {
        let ingredient = Ingredient::new_by_line(";").unwrap();
        assert_eq!(ingredient.name, "");
        assert_eq!(ingredient.group, Group::Other);
    }

    #[test]
    fn test_new_by_line_only_name_1() {
        let ingredient = Ingredient::new_by_line("Salami").unwrap();
        assert_eq!(ingredient.name, "Salami");
        assert_eq!(ingredient.group, Group::Other);
    }

    #[test]
    fn test_new_by_line_only_name_2() {
        let ingredient = Ingredient::new_by_line("Salami;").unwrap();
        assert_eq!(ingredient.name, "Salami");
        assert_eq!(ingredient.group, Group::Other);
    }

    #[test]
    fn test_new_by_line_only_name_3() {
        let ingredient = Ingredient::new_by_line("Salami;;").unwrap();
        assert_eq!(ingredient.name, "Salami");
        assert_eq!(ingredient.group, Group::Other);
    }

    #[test]
    fn test_new_by_line_only_group_1() {
        let ingredient = Ingredient::new_by_line(";0").unwrap();
        assert_eq!(ingredient.name, "");
        assert_eq!(ingredient.group, Group::Vegetable);
    }

    #[test]
    fn test_new_by_line_invalid_group_1() {
        let ingredient = Ingredient::new_by_line("Salami;-1").unwrap();
        assert_eq!(ingredient.name, "Salami");
        assert_eq!(ingredient.group, Group::Other);
    }

    #[test]
    fn test_new_by_line_invalid_group_2() {
        let ingredient = Ingredient::new_by_line("Salami;asd").unwrap();
        assert_eq!(ingredient.name, "Salami");
        assert_eq!(ingredient.group, Group::Other);
    }

    #[test]
    fn test_sort_1() {
        let i1 = Ingredient {
            name: String::from("asd"),
            group: Group::Other,
            preferred_store: Store::Any,
        };
        let i2 = Ingredient {
            name: String::from("asd"),
            group: Group::Other,
            preferred_store: Store::Any,
        };

        assert_eq!(i1.cmp(&i2), Ordering::Equal);
    }

    #[test]
    fn test_sort_2() {
        let i1 = Ingredient {
            name: String::from("asd"),
            group: Group::Vegetable,
            preferred_store: Store::Any,
        };
        let i2 = Ingredient {
            name: String::from("asd"),
            group: Group::Other,
            preferred_store: Store::Any,
        };

        assert_eq!(i1.cmp(&i2), Ordering::Greater);
    }

    #[test]
    fn test_sort_3() {
        let i1 = Ingredient {
            name: String::from("asd"),
            group: Group::Vegetable,
            preferred_store: Store::Any,
        };
        let i2 = Ingredient {
            name: String::from("asc"),
            group: Group::Vegetable,
            preferred_store: Store::Any,
        };

        assert_eq!(i1.cmp(&i2), Ordering::Greater);
    }
}
