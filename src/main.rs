use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

mod cooking_book {
    pub mod group;
    pub mod ingredient;
    pub mod recipe;
    pub mod shopping_list;
    pub mod store;
}

mod file_access {
    pub mod persistency;
}
use crate::file_access::persistency;
use crate::cooking_book::group::Group;
use crate::cooking_book::ingredient::Ingredient;
use crate::cooking_book::recipe::Recipe;
use crate::cooking_book::shopping_list::ShoppingList;
use crate::cooking_book::store::Store;

fn main() {
    web();
    cli();
}

/// Read from stdin
///
/// # Panics
///
/// The `read_from_stdin` function will panic if it cannot read from stdin
pub fn read_from_stdin() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Couldn't read from stdin");
    return input.trim().to_string();
}

fn web() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let get_shopping_list = b"GET /getShoppingList HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) || buffer.starts_with(get_shopping_list) {
        ("HTTP/1.1 200 OK\r\n\r\n", "web/index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "web/404.html")
    };

    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();

    if buffer.starts_with(get_shopping_list) {
        let shopping_list = persistency::load_shopping_list();
        contents = shopping_list.to_json();
    } else {
        file.read_to_string(&mut contents).unwrap();
    }

    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn cli() {
    loop {
        print_menu();
        let main_menu = read_from_stdin();

        match main_menu.as_str() {
            "0" => break,
            "1" => loop {
                print_menu_ingredients();
                let sub_menu = read_from_stdin();

                match sub_menu.as_str() {
                    "0" => break,
                    "1" => Ingredient::print_all_ingredients_multi_line(),
                    "2" => Ingredient::save_new_ingredient().unwrap_or_else(|e| eprintln!("{}", e)),
                    "3" => Ingredient::delete_ingredient().unwrap_or_else(|e| eprintln!("{}", e)),
                    &_ => continue,
                }
            },
            "2" => loop {
                print_menu_recipes();
                let sub_menu = read_from_stdin();

                match sub_menu.as_str() {
                    "0" => break,
                    "1" => Recipe::print_all_recipes_multi_line(),
                    "2" => Recipe::print_recipes_by_name(),
                    "3" => Recipe::print_recipes_by_used_ingredient(),
                    "4" => Recipe::print_recipes_by_tag(),
                    &_ => continue,
                }
            },
            "3" => loop {
                print_menu_shopping_list();
                let sub_menu = read_from_stdin();

                match sub_menu.as_str() {
                    "0" => break,
                    "1" => ShoppingList::print_shopping_list(),
                    "2" => ShoppingList::add_ingredient(),
                    "3" => ShoppingList::remove_ingredient(),
                    &_ => continue,
                }
            },
            "4" => Group::print_all_groups_multi_line(),
            "5" => Store::print_all_stores_multi_line(),
            &_ => eprintln!("Unrecognized input {}", main_menu),
        }
    }
}

fn print_menu() {
    println!("-------------------");
    println!("0: Exit");
    println!("1: Ingredients");
    println!("2: Recipes");
    println!("3: Shopping List");
    println!("4: Show all Groups");
    println!("5: Show all Stores");
    println!("-------------------");
}

fn print_menu_ingredients() {
    println!("-------------------");
    println!("0: Back");
    println!("1: Show all Ingredients");
    println!("2: Add Ingredient");
    println!("3: Remove Ingredient");
    println!("-------------------");
}

fn print_menu_recipes() {
    println!("-------------------");
    println!("0: Back");
    println!("1: Show all Recipes");
    println!("2: Show Recipes by Name");
    println!("3: Show Recipes by Ingredient");
    println!("4: Show Recipes by Tag");
    println!("-------------------");
}

fn print_menu_shopping_list() {
    println!("-------------------");
    println!("0: Back");
    println!("1: Show Shopping List");
    println!("2: Add Item");
    println!("3: Remove Item");
    println!("-------------------");
}
