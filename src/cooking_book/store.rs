use self::Store::*;
use std::slice::Iter;

/// The available stores for shopping.
#[derive(Eq, Ord, PartialEq, PartialOrd, Clone, Debug, Copy, Hash)]
pub enum Store {
    Rewe    = 0,
    DM      = 1,
    Denz    = 2,
    Any     = -1,
}

impl Store {
    fn get_store_iterator() -> Iter<'static, Store> {
        static STORES: [Store; 4] = [Rewe, DM, Denz, Any];
        STORES.into_iter()
    }

    pub fn print_all_stores_multi_line() {
        let mut idx = 0;
        for store in Store::get_store_iterator() {
            println!("{}: {:?}", idx, store);
            idx += 1;
        }
    }

    pub fn print_all_stores_single_line() {
        let mut idx = 0;
        for store in Store::get_store_iterator() {
            print!("{}: {:?}\t", idx, store);
            idx += 1;
        }
        println!();
    }

    /// Returns the decoded Store.
    /// 
    /// #Arguments
    /// * `number` The encoded store.
    pub fn lookup_store_number(number: usize) -> Store {
        match number {
            0 => Store::Rewe,
            1 => Store::DM,
            2 => Store::Denz,
            _ => Store::Any,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Store;
    #[test]
    fn test_store_size() {
        let it = Store::get_store_iterator();
        let (size, _asd) = it.size_hint();
        assert_eq!(size, 4);
    }

    #[test]
    fn test_lookup_store() {
        assert_eq!(Store::lookup_store_number(0), Store::Rewe);
        assert_eq!(Store::lookup_store_number(1), Store::DM);
        assert_eq!(Store::lookup_store_number(2), Store::Denz);
        assert_eq!(Store::lookup_store_number(3), Store::Any);
        assert_eq!(Store::lookup_store_number(4), Store::Any);
    }
}
