use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt;

use crate::cooking_book::group::Group;
use crate::cooking_book::store::Store;
use crate::file_access::persistency;

/// An ingredient which is in a specific group and has a preferred store
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Ingredient {
    pub name: String,
    pub group: Group,
    pub preferred_store: Store,
}

impl Ingredient {
    pub fn new_by_line(line: &str) -> Ingredient {
        let mut values = line.split(';');

        let name = String::from(values.next().unwrap());
        let group = match values.next() {
            Some(group) => group.trim(),
            None => "<empty>",
        };
        let group: Group = match group.parse::<usize>() {
            Ok(num) => Group::lookup_group_number(num),
            Err(_) => Group::Other,
        };

        let store = match values.next() {
            Some(store) => store.trim(),
            None => "<empty>",
        };
        let store: Store = match store.parse::<usize>() {
            Ok(num) => Store::lookup_store_number(num),
            Err(_) => Store::Any,
        };

        Ingredient {
            name,
            group,
            preferred_store: store,
        }
    }

    /// Change the assigned group
    /// 
    /// #Arguments
    /// 
    /// * `group` the encoded group
    pub fn set_group(&mut self, group: usize) {
        self.group = Group::lookup_group_number(group);
    }    
    
    /// Change the assigned store
    /// 
    /// #Arguments
    /// 
    /// * `store` the encoded store
    pub fn set_store(&mut self, store: usize) {
        self.preferred_store = Store::lookup_store_number(store);
    }

    /// Create a new ingredient with only a name. Store and Group will fallback to their defaults.
    ///
    /// #Arguments
    ///
    /// * `name` The name of the new ingredient.
    pub fn new_by_name(name: String) -> Ingredient {
        return Ingredient {
            name: name.to_string(),
            group: Group::Other,
            preferred_store: Store::Any,
        };
    }
    /// Create a new ingredient with only a name. Store and Group will fallback to their defaults.
    /// The new ingredient will be persisted.
    ///
    /// #Arguments
    ///
    /// * `name` The name of the new ingredient.
    pub fn persist_new_ingredient(
        name: &String,
        all_ingredients: &mut HashMap<String, Ingredient>,
    ) -> Result<(), String> {
        let new_ingredient = Ingredient::new_by_name(name.to_string());
        let result = persistency::write_single_ingredient(&new_ingredient);
        if result.is_err() {
            return result;
        }
        all_ingredients.insert(name.to_string(), new_ingredient);
        return Ok(());
    }

    /// Export the list of all ingredients to JSON.
    ///
    /// #Arguments
    ///
    /// ' `all_ingredients` The collection of all ingredients.
    pub fn all_to_json(all_ingredients: &HashMap<String, Ingredient>) -> String {
        let mut json: String = String::new();
        json.push('[');

        let mut keys: Vec<&String> = all_ingredients.keys().collect();
        keys.sort();

        let mut is_first: bool = true;
        for k in keys {
            if !is_first {
                json.push_str(", ");
            }

            json.push_str(&all_ingredients.get(k).unwrap().to_json());
            is_first = false;
        }

        json.push(']');
        return json;
    }

    /// Export this ingredient to JSON.
    pub fn to_json(&self) -> String {
        let mut json: String = String::new();
        json.push('{');

        json.push_str("\"name\": \"");
        json.push_str(&self.name);
        json.push_str("\", ");

        json.push_str("\"group\": \"");
        json.push_str(&format!("{}", &self.group));
        json.push_str("\", ");

        json.push_str("\"store\": \"");
        json.push_str(&format!("{}", &self.preferred_store));
        json.push_str("\"");

        json.push('}');

        return json;
    }

    /// Returns the list of all ingredients.
    pub fn get_all_ingredients() -> Vec<Ingredient> {
        let all_ingredients = persistency::load_ingredients();
        let mut vec_ingredients: Vec<Ingredient> = Vec::new();

        for (_key, value) in all_ingredients.iter() {
            vec_ingredients.push(value.clone());
        }
        vec_ingredients.sort();

        return vec_ingredients;
    }
}

impl PartialOrd for Ingredient {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Ingredient {
    fn cmp(&self, other: &Self) -> Ordering {
        let order_store = self.preferred_store.cmp(&other.preferred_store);

        if order_store == Ordering::Equal {
            let order_group = self.group.cmp(&other.group);
            if order_group == Ordering::Equal {
                return self.name.cmp(&other.name);
            }
            return order_group;
        }
        return order_store;
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
        let ingredient = Ingredient {
            name: "Gurke".to_string(),
            group: Group::Vegetable,
            preferred_store: Store::Any,
        };
        assert_eq!(
            ingredient.to_json(),
            "{\"name\": \"Gurke\", \"group\": \"Gemüse\", \"store\": \"Überall\"}"
        );
    }

    #[test]
    fn test_new_by_line_empty_1() {
        let ingredient = Ingredient::new_by_line("");
        assert_eq!(ingredient.name, "");
        assert_eq!(ingredient.group, Group::Other);
    }

    #[test]
    fn test_new_by_line_empty_2() {
        let ingredient = Ingredient::new_by_line(";");
        assert_eq!(ingredient.name, "");
        assert_eq!(ingredient.group, Group::Other);
    }

    #[test]
    fn test_new_by_line_only_name_1() {
        let ingredient = Ingredient::new_by_line("Salami");
        assert_eq!(ingredient.name, "Salami");
        assert_eq!(ingredient.group, Group::Other);
    }

    #[test]
    fn test_new_by_line_only_name_2() {
        let ingredient = Ingredient::new_by_line("Salami;");
        assert_eq!(ingredient.name, "Salami");
        assert_eq!(ingredient.group, Group::Other);
    }

    #[test]
    fn test_new_by_line_only_name_3() {
        let ingredient = Ingredient::new_by_line("Salami;;");
        assert_eq!(ingredient.name, "Salami");
        assert_eq!(ingredient.group, Group::Other);
    }

    #[test]
    fn test_new_by_line_only_group_1() {
        let ingredient = Ingredient::new_by_line(";0");
        assert_eq!(ingredient.name, "");
        assert_eq!(ingredient.group, Group::Vegetable);
    }

    #[test]
    fn test_new_by_line_invalid_group_1() {
        let ingredient = Ingredient::new_by_line("Salami;-1");
        assert_eq!(ingredient.name, "Salami");
        assert_eq!(ingredient.group, Group::Other);
    }

    #[test]
    fn test_new_by_line_invalid_group_2() {
        let ingredient = Ingredient::new_by_line("Salami;asd");
        assert_eq!(ingredient.name, "Salami");
        assert_eq!(ingredient.group, Group::Other);
    }

    #[test]
    fn test_sort_equal() {
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
    fn test_sort_group() {
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
    fn test_sort_name() {
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

    #[test]
    fn test_sort_store() {
        let i1 = Ingredient {
            name: String::from("asd"),
            group: Group::Vegetable,
            preferred_store: Store::Any,
        };
        let i2 = Ingredient {
            name: String::from("asd"),
            group: Group::Vegetable,
            preferred_store: Store::DM,
        };

        assert_eq!(i1.cmp(&i2), Ordering::Less);
    }
}
