pub mod item_kind;
pub mod item_name;
pub mod item_weight;

use crate::{error::InventoryError, rarity::Rarity};
use item_kind::ItemKind;
use item_name::ItemName;
use item_weight::ItemWeight;
use std::fmt::{Debug, Formatter};

pub struct Item {
    name: ItemName,
    kind: ItemKind,
    rarity: Rarity,
    weight: ItemWeight,
}

impl Item {
    pub fn new(
        name: &str,
        kind: ItemKind,
        rarity: Rarity,
        weight: u32,
    ) -> Result<Self, InventoryError> {
        let name = match ItemName::try_from(name) {
            Ok(name) => name,
            Err(error) => return Err(InventoryError::from(error)),
        };
        let weight = match ItemWeight::try_from(weight) {
            Ok(weight) => weight,
            Err(error) => return Err(InventoryError::from(error)),
        };

        Ok(Self {
            name,
            kind,
            rarity,
            weight,
        })
    }

    pub fn name(&self) -> &str {
        self.name.value()
    }

    pub fn kind(&self) -> &ItemKind {
        &self.kind
    }

    pub fn rarity(&self) -> Rarity {
        self.rarity
    }

    pub fn weight(&self) -> u32 {
        self.weight.value()
    }
}

impl Debug for Item {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "Item {{ name: {:?}, kind: {:?}, rarity: {:?}, weight: {:?} }}",
            self.name, self.kind, self.rarity, self.weight,
        )
    }
}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.kind == other.kind
            && self.rarity == other.rarity
            && self.weight == other.weight
    }
}

#[allow(unused)]
impl Item {
    fn describe(&self) -> String {
        format!(
            "{:?} is item with {} type and a weight of {:?}, and has a {:?} rarity",
            self.name, self.kind, self.weight, self.rarity
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sword() -> Item {
        Item::new("Sword", ItemKind::Weapon { damage: 50 }, Rarity::Epic, 5).unwrap()
    }

    fn potion() -> Item {
        Item::new(
            "Health Potion",
            ItemKind::Potion { healing: 25 },
            Rarity::Common,
            1,
        )
        .unwrap()
    }

    #[test]
    fn describe_weapon_contains_expected_fields() {
        let s = sword().describe();
        assert!(s.contains("Sword"));
        assert!(s.contains("Epic"));
        assert!(s.contains("Weapon"));
        assert!(s.contains("50"));
        assert!(s.contains("5"));
    }

    #[test]
    fn describe_potion_contains_expected_fields() {
        let s = potion().describe();
        assert!(s.contains("Health Potion"));
        assert!(s.contains("Common"));
        assert!(s.contains("Potion"));
        assert!(s.contains("25"));
    }

    // =====================================================================
    // PHASE 1 — Encapsulation: Item getters
    // =====================================================================

    #[test]
    fn item_getters_return_field_values() {
        let s = sword();
        assert_eq!(s.name(), "Sword");
        assert_eq!(s.weight(), 5);
        assert_eq!(s.rarity(), Rarity::Epic);
        match s.kind() {
            ItemKind::Weapon { damage } => assert_eq!(*damage, 50),
            _ => panic!("expected a Weapon"),
        }
    }

    // =====================================================================
    // PHASE 2 — Item::new validated constructor
    // =====================================================================

    #[test]
    fn item_new_returns_ok_for_valid_input() {
        let item = Item::new("Sword", ItemKind::Weapon { damage: 50 }, Rarity::Epic, 5);
        assert!(item.is_ok());
    }

    #[test]
    fn item_new_rejects_empty_name() {
        let item = Item::new(
            String::new().as_str(),
            ItemKind::Weapon { damage: 50 },
            Rarity::Epic,
            5,
        );
        assert!(item.is_err());
    }

    #[test]
    fn item_new_rejects_zero_weight() {
        let item = Item::new("Sword", ItemKind::Weapon { damage: 50 }, Rarity::Epic, 0);
        assert!(item.is_err());
    }

    // =====================================================================
    // PHASE 6 — TryFrom / From conversions
    // =====================================================================

    // #[test]
    // fn rarity_try_from_parses_known_variants() {
    //     assert_eq!(Rarity::try_from("common").unwrap(), Rarity::Common);
    //     assert_eq!(Rarity::try_from("rare").unwrap(), Rarity::Rare);
    //     assert_eq!(Rarity::try_from("epic").unwrap(), Rarity::Epic);
    //     assert_eq!(Rarity::try_from("legendary").unwrap(), Rarity::Legendary);
    // }

    // #[test]
    // fn rarity_try_from_rejects_unknown_value() {
    //     assert!(Rarity::try_from("mythic").is_err());
    //     assert!(Rarity::try_from("").is_err());
    // }

    // #[test]
    // fn item_kind_try_from_parses_weapon() {
    //     let kind = ItemKind::try_from("weapon:50").unwrap();
    //     assert_eq!(kind, ItemKind::Weapon { damage: 50 });
    // }

    // #[test]
    // fn item_kind_try_from_parses_potion_and_armor() {
    //     assert_eq!(
    //         ItemKind::try_from("potion:25").unwrap(),
    //         ItemKind::Potion { healing: 25 }
    //     );
    //     assert_eq!(
    //         ItemKind::try_from("armor:30").unwrap(),
    //         ItemKind::Armor { defense: 30 }
    //     );
    // }

    // #[test]
    // fn item_kind_try_from_rejects_garbage() {
    //     assert!(ItemKind::try_from("nope").is_err());
    //     assert!(ItemKind::try_from("weapon:abc").is_err());
    //     assert!(ItemKind::try_from("weapon").is_err());
    // }

    // #[test]
    // fn item_summary_from_item_copies_name_and_value() {
    //     let s = sword();
    //     let summary: ItemSummary = (&s).into();
    //     assert_eq!(summary.name, "Sword");
    //     // adapt this assertion to your chosen value formula
    //     assert!(summary.value > 0);
    // }

    // =====================================================================
    // PHASE 7 — Newtype ItemName + Deref + ?Sized
    // =====================================================================

    // #[test]
    // fn item_name_from_str_wraps_owned_string() {
    //     let name: ItemName = "Sword".into();
    //     let _borrowed: &str = &name;
    // }

    // #[test]
    // fn item_name_from_string_wraps_owned_string() {
    //     let owned: String = "Sword".to_string();
    //     let name: ItemName = owned.into();
    //     assert_eq!(&*name, "Sword");
    // }

    // #[test]
    // fn item_name_derefs_to_str_methods() {
    //     let name: ItemName = "Sword".into();
    //     assert_eq!(name.len(), 5);
    //     assert!(name.starts_with("Sw"));
    //     assert!(name.contains("wor"));
    // }

    // #[test]
    // fn length_of_accepts_str_string_and_item_name() {
    //     let raw: &str = "abc";
    //     let owned: String = String::from("abcd");
    //     let name: ItemName = "abcde".into();
    //     assert_eq!(length_of(raw), 3);
    //     assert_eq!(length_of(&owned), 4);
    //     assert_eq!(length_of(&name), 5);
    // }

    // =====================================================================
    // PHASE 11 — Rarity ordering (PartialOrd / Ord)
    // =====================================================================

    // #[test]
    // fn rarity_orders_common_below_legendary() {
    //     assert!(Rarity::Common < Rarity::Rare);
    //     assert!(Rarity::Rare < Rarity::Epic);
    //     assert!(Rarity::Epic < Rarity::Legendary);
    // }

    // #[test]
    // fn rarity_sort_ascending_puts_common_first() {
    //     let mut v = vec![
    //         Rarity::Legendary,
    //         Rarity::Common,
    //         Rarity::Epic,
    //         Rarity::Rare,
    //     ];
    //     v.sort();
    //     assert_eq!(
    //         v,
    //         vec![
    //             Rarity::Common,
    //             Rarity::Rare,
    //             Rarity::Epic,
    //             Rarity::Legendary,
    //         ]
    //     );
    // }

    // =====================================================================
    // PHASE 14 — Item::parse_compact (let-else)
    // =====================================================================

    // #[test]
    // fn parse_compact_round_trips_a_sword() {
    //     let item = Item::parse_compact("Sword|weapon:50|epic|5").unwrap();
    //     assert_eq!(item.name(), "Sword");
    //     assert_eq!(item.weight(), 5);
    //     assert_eq!(*item.rarity(), Rarity::Epic);
    //     assert_eq!(*item.kind(), ItemKind::Weapon { damage: 50 });
    // }

    // #[test]
    // fn parse_compact_parses_potion_and_armor() {
    //     let p = Item::parse_compact("Mana|potion:25|common|1").unwrap();
    //     assert_eq!(*p.kind(), ItemKind::Potion { healing: 25 });
    //     let a = Item::parse_compact("Plate|armor:30|rare|10").unwrap();
    //     assert_eq!(*a.kind(), ItemKind::Armor { defense: 30 });
    // }

    // #[test]
    // fn parse_compact_rejects_malformed_input() {
    //     assert!(Item::parse_compact("garbage").is_err());
    //     assert!(Item::parse_compact("Sword|weapon|epic|5").is_err());
    //     assert!(Item::parse_compact("Sword|weapon:50|epic|notanumber").is_err());
    //     assert!(Item::parse_compact("Sword|weapon:50|mythic|5").is_err());
    //     assert!(Item::parse_compact("").is_err());
    // }
}
