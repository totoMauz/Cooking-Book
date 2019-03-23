use self::Group::*;
use std::fmt;
use std::slice::Iter;

/// The groups for ingredients
#[derive(Eq, Ord, PartialEq, PartialOrd, Clone, Debug, Copy, Hash)]
pub enum Group {
    Vegetable = 0,
    Fruit = 1,
    Freezer = 2,
    Conserved = 3,
    Beverage = 4,
    Baking = 5,
    Pasta = 6,
    Legume = 7,
    Spice = 8,
    Snacks = 9,
    Sweets = 10,
    Other = -1,
}

impl Group {
    fn get_group_iterator() -> Iter<'static, Group> {
        static GROUPS: [Group; 8] = [
            Vegetable, Fruit, Freezer, Conserved, Beverage, Baking, Spice, Other,
        ];
        GROUPS.into_iter()
    }

    /// Returns the decoded Group.
    ///
    /// #Arguments
    /// * `number` The encoded Group.
    pub fn lookup_group_number(number: usize) -> Group {
        match number {
            0 => Group::Vegetable,
            1 => Group::Fruit,
            2 => Group::Freezer,
            3 => Group::Conserved,
            4 => Group::Beverage,
            5 => Group::Baking,
            6 => Group::Pasta,
            7 => Group::Legume,
            8 => Group::Spice,
            9 => Group::Snacks,
            10 => Group::Sweets,
            _ => Group::Other,
        }
    }
}

impl fmt::Display for Group {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Group::Vegetable => write!(f, "Gemüse"),
            Group::Fruit => write!(f, "Obst"),
            Group::Freezer => write!(f, "Kühlung"),
            Group::Conserved => write!(f, "Konserve"),
            Group::Beverage => write!(f, "Getränk"),
            Group::Baking => write!(f, "Backzutat"),
            Group::Pasta => write!(f, "Nudeln"),
            Group::Legume => write!(f, "Hülsenfrüchte"),
            Group::Spice => write!(f, "Gewürz"),
            Group::Snacks => write!(f, "Knabberkram"),
            Group::Sweets => write!(f, "Süßigkeiten"),
            Group::Other => write!(f, "Anderes"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Group;
    #[test]
    fn test_group_size() {
        let it = Group::get_group_iterator();
        let (size, _asd) = it.size_hint();
        assert_eq!(size, 8);
    }

    #[test]
    fn test_group_display() {
        assert_eq!(&format!("{}", Group::Other), "Anderes");
    }

    #[test]
    fn test_lookup_group() {
        assert_eq!(Group::lookup_group_number(0), Group::Vegetable);
        assert_eq!(Group::lookup_group_number(1), Group::Fruit);
        assert_eq!(Group::lookup_group_number(2), Group::Freezer);
        assert_eq!(Group::lookup_group_number(3), Group::Conserved);
        assert_eq!(Group::lookup_group_number(4), Group::Beverage);
        assert_eq!(Group::lookup_group_number(5), Group::Baking);
        assert_eq!(Group::lookup_group_number(6), Group::Pasta);
        assert_eq!(Group::lookup_group_number(7), Group::Legume);
        assert_eq!(Group::lookup_group_number(8), Group::Spice);
        assert_eq!(Group::lookup_group_number(9), Group::Snacks);
        assert_eq!(Group::lookup_group_number(10), Group::Sweets);
        assert_eq!(Group::lookup_group_number(11), Group::Other);
    }
}
