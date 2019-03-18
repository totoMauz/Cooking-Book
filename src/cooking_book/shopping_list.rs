use std::collections::HashMap;

use crate::file_access::persistency;
use crate::Ingredient;

/// The shopping list
#[derive(PartialEq, Eq)]
pub struct ShoppingList {
    pub to_buy: HashMap<Ingredient, u16>,
}

impl ShoppingList {
    pub fn new() -> ShoppingList {
        let to_buy: HashMap<Ingredient, u16> = HashMap::new();
        return ShoppingList { to_buy };
    }

    pub fn add_ingredient() {
        println!("Enter the name of the ingredient to add");
        let name = crate::read_from_stdin();
        let mut all_ingredients = persistency::load_ingredients();

        if !all_ingredients.contains_key(&name) {
            if Ingredient::persist_new_ingredient(&name, &mut all_ingredients).is_err() {
                eprintln!("Couldn't persist new ingredient");
            }
        }

        let ingredient: &Ingredient = all_ingredients.get(&name).unwrap();

        let mut shopping_list = persistency::load_shopping_list();
        shopping_list.add_and_save(ingredient).unwrap_or_else(|e| eprintln!("{}", e));
    }

    /// Add an item to the shopping list. If the item is already present, the number to buy will be incremented.
    /// The updated shopping list will be persisted.
    pub fn add_and_save(&mut self, ingredient : &Ingredient) -> Result<(), String> {
        self.add_or_increment(ingredient);
        return persistency::write_shopping_list(&self);
    }

    fn add_or_increment(&mut self, ingredient: &Ingredient) {
        let amount = self.to_buy.entry(ingredient.clone()).or_insert(0);
        *amount += 1;
    }

    /// Add an item with amount to the shopping list.
    pub fn add_item(&mut self, ingredient: Ingredient, amount: u16) {
        self.to_buy.insert(ingredient, amount);
    }

    pub fn remove_ingredient() {
        println!("Enter the name of the ingredient to remove");
        let name = crate::read_from_stdin();
        let mut all_ingredients = persistency::load_ingredients();

        if !all_ingredients.contains_key(&name) {
            Ingredient::persist_new_ingredient(&name, &mut all_ingredients)
                .unwrap_or_else(|e| eprintln!("{}", e));
        }

        let ingredient: &Ingredient = all_ingredients.get(&name).unwrap();

        let mut shopping_list = persistency::load_shopping_list();
        shopping_list.remove_and_save(ingredient).unwrap_or_else(|e| eprintln!("{}", e));
    }

    /// Remove an item from the shopping list. The updated shopping list will be persisted.
    /// 
    /// #Arguments
    /// 
    /// * `ingredient` The ingredient to remove from the list.
    pub fn remove_and_save(&mut self, ingredient: &Ingredient) -> Result<(), String> {
        self.remove(ingredient);
        return persistency::write_shopping_list(&self);
    }

    fn remove(&mut self, ingredient: &Ingredient) {
        self.to_buy.remove(&ingredient);
    }

    pub fn print_shopping_list() {
        let shopping_list = persistency::load_shopping_list();

        let mut keys: Vec<&Ingredient> = shopping_list.to_buy.keys().collect();
        keys.sort();

        for k in keys {
            let a = shopping_list.to_buy.get(k).unwrap();
            println!("{}:\t{}", k.name, a);
        }
    }

    /// Exports the shopping list to json.
    pub fn to_json(&self) -> String {
        let mut keys: Vec<&Ingredient> = self.to_buy.keys().collect();
        keys.sort();

        let first_entry = keys.first().unwrap();
        let store = first_entry.preferred_store;
        let category = first_entry.group;

        let mut json: String = String::new();
        json.push_str("{\"");
        json.push_str(&format!("{:?}", store));
        json.push_str("\": {\"");

        json.push_str(&format!("{:}", category));
        json.push_str("\": [");

        let mut is_first: bool = true;
        for i in keys {

            if i.preferred_store != store {
                let store = i.preferred_store;
                let category = i.group;
                json.push_str("]}, \"");
                json.push_str(&format!("{:?}", store));
                json.push_str("\": {\"");
                json.push_str(&format!("{:}", category));
                json.push_str("\": [");
                is_first = true;
            }


            if i.group != category {
                let category = i.group;
                json.push_str("], \"");
                json.push_str(&format!("{:}", category));
                json.push_str("\": [");
                is_first = true;
            }

            if !is_first {
                json.push_str(", ");
            }

            json.push_str("{\"name\": \"");
            json.push_str(&i.name);
            json.push_str("\"");

            let amount: &u16 = self.to_buy.get(i).unwrap();
            if amount > &1u16 {
                json.push_str(&format!(", \"amount\": {}", amount));
            }

            json.push('}');
            is_first = false;
        }

        json.push_str("]}}");
        return json;
    }
}

#[cfg(test)]
mod tests {
    use super::ShoppingList;
    use crate::cooking_book::group::Group;
    use crate::cooking_book::ingredient::Ingredient;
    use crate::cooking_book::store::Store;

    #[test]
    fn test_add_or_increment() {
        let ingredient = Ingredient::new_by_name("Banane".to_string());
        let mut shopping_list = ShoppingList::new();
        assert!(shopping_list.to_buy.is_empty());

        shopping_list.add_or_increment(&ingredient);
        assert!(shopping_list.to_buy.contains_key(&ingredient));

        let mut expected_count: u16 = 1;
        assert_eq!(
            shopping_list.to_buy.get(&ingredient).unwrap(),
            &expected_count
        );

        shopping_list.add_or_increment(&ingredient);
        expected_count += 1;
        assert_eq!(
            shopping_list.to_buy.get(&ingredient).unwrap(),
            &expected_count
        );
    }

    #[test]
    fn test_remove() {
        let ingredient = Ingredient::new_by_name("Banane".to_string());
        let mut shopping_list = ShoppingList::new();
        assert!(shopping_list.to_buy.is_empty());

        shopping_list.add_or_increment(&ingredient);
        assert!(shopping_list.to_buy.contains_key(&ingredient));

        shopping_list.remove(&ingredient);
        assert!(shopping_list.to_buy.is_empty());

        shopping_list.remove(&ingredient);
        assert!(shopping_list.to_buy.is_empty());
    }

    #[test]
    fn test_to_json_1() {
        let ingredient1 = Ingredient::new_by_name("Banane".to_string());
        let ingredient2 = Ingredient::new_by_name("Gurke".to_string());
        let mut shopping_list = ShoppingList::new();
        shopping_list.add_or_increment(&ingredient1);
        shopping_list.add_or_increment(&ingredient2);

        assert_eq!(
            shopping_list.to_json(),
            "{\"Any\": {\"Anderes\": [{\"name\": \"Banane\"}, {\"name\": \"Gurke\"}]}}"
        );
    }

    #[test]
    fn test_to_json_2() {
        let ingredient1 = Ingredient::new_by_name("Banane".to_string());
        let ingredient2 = Ingredient::new_by_name("Gurke".to_string());
        let mut shopping_list = ShoppingList::new();
        shopping_list.add_or_increment(&ingredient1);
        shopping_list.add_or_increment(&ingredient2);
        shopping_list.add_or_increment(&ingredient2);

        assert_eq!(
            shopping_list.to_json(),
            "{\"Any\": {\"Anderes\": [{\"name\": \"Banane\"}, {\"name\": \"Gurke\", \"amount\": 2}]}}"
        );
    }

    #[test]
    fn test_to_json_3() {
        let ingredient1 = Ingredient {
            name: "Banane".to_string(),
            group: Group::Fruit,
            preferred_store: Store::Any,
        };
        let ingredient2 = Ingredient {
            name: "Gurke".to_string(),
            group: Group::Vegetable,
            preferred_store: Store::Any,
        };
        let mut shopping_list = ShoppingList::new();
        shopping_list.add_or_increment(&ingredient1);
        shopping_list.add_or_increment(&ingredient2);

        assert_eq!(
            shopping_list.to_json(),
            "{\"Any\": {\"Gemüse\": [{\"name\": \"Gurke\"}], \"Obst\": [{\"name\": \"Banane\"}]}}"
        );
    }

    #[test]
    fn test_to_json_4() {
        let ingredient1 = Ingredient {
            name: "Banane".to_string(),
            group: Group::Other,
            preferred_store: Store::Lidl,
        };
        let ingredient2 = Ingredient {
            name: "Gurke".to_string(),
            group: Group::Other,
            preferred_store: Store::DM,
        };
        let mut shopping_list = ShoppingList::new();
        shopping_list.add_or_increment(&ingredient1);
        shopping_list.add_or_increment(&ingredient2);

        assert_eq!(
            shopping_list.to_json(),
            "{\"Lidl\": {\"Anderes\": [{\"name\": \"Banane\"}]}, \"DM\": {\"Anderes\": [{\"name\": \"Gurke\"}]}}"
        );
    }
}
