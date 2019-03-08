use self::Store::*;
use std::slice::Iter;

#[derive(Eq, Ord, PartialEq, PartialOrd, Clone, Debug, Copy)]
pub enum Store {
    Lidl = 0,
    ALDI = 1,
    Rewe = 2,
    DM = 3,
    Denz = 4,
    Netto = 5,
    Kaufland = 6,
    Any = -1,
}

impl Store {
    fn get_store_iterator() -> Iter<'static, Store> {
        static STORES: [Store; 8] = [Lidl, ALDI, Rewe, DM, Denz, Netto, Kaufland, Any];
        STORES.into_iter()
    }

    pub fn print_all_store_multi_line() {
        let mut idx = 0;
        for store in Store::get_store_iterator() {
            println!("{}: {:?}", idx, store);
            idx += 1;
        }
    }

    pub fn lookup_store_number(number: usize) -> Store {
        match number {
            0 => Store::Lidl,
            1 => Store::ALDI,
            2 => Store::Rewe,
            3 => Store::DM,
            4 => Store::Denz,
            5 => Store::Netto,
            6 => Store::Kaufland,
            _ => Store::Any,
        }
    }
}
