use crate::error::InventoryError;
use crate::item::{Item, ItemKind, Rarity};

pub struct Backpack {
    pub items: Vec<Item>,
    pub max_weight: u32,
}

impl Backpack {
    pub fn new(max_weight: u32) -> Self {
        // TODO (FR): construit un Backpack vide avec la capacité max donnée.
        Self {
            items: Vec::new(),
            max_weight,
        }
    }

    pub fn add_item(&mut self, item: Item) -> Result<(), InventoryError> {
        let total_weight = self.total_weight();
        if self.max_weight - total_weight >= item.weight {
            self.items.push(item);
            Ok(())
        } else {
            Err(InventoryError::BackpackFull)
        }
    }

    pub fn remove_item(&mut self, name: &str) -> Result<Item, InventoryError> {
        let mut index_found: Option<usize> = None;
        for (index, item) in self.items.iter().enumerate() {
            if item.name == name {
                index_found = Some(index);
            }
        }

        if index_found.is_some() {
            Ok(self.items.remove(index_found.unwrap()))
        } else {
            Err(InventoryError::ItemNotFound)
        }
    }

    pub fn total_weight(&self) -> u32 {
        let mut total_weight = 0;
        for item in &self.items {
            total_weight += item.weight;
        }
        total_weight
    }

    pub fn count_by_rarity(&self, rarity: Rarity) -> usize {
        let mut count: usize = 0;
        for item in self.items.iter() {
            if item.rarity == rarity {
                count += 1;
            }
        }
        count
    }

    pub fn strongest_weapon(&self) -> Option<&Item> {
        let mut strongest_weapon: Option<&Item> = None;
        for item in self.items.iter() {
            match item.kind {
                ItemKind::Weapon {
                    damage: current_damage,
                } => match strongest_weapon {
                    Some(weapon) => {
                        if let ItemKind::Weapon {
                            damage: strongest_damage,
                        } = weapon.kind
                        {
                            if strongest_damage < current_damage {
                                strongest_weapon = Some(item)
                            }
                        }
                    }
                    None => strongest_weapon = Some(item),
                },
                _ => (),
            }
        }
        strongest_weapon
    }

    pub fn total_value(&self) -> u32 {
        let mut total_value = 0;
        for item in self.items.iter() {
            let value = match item.rarity {
                Rarity::Common => 1,
                Rarity::Rare => 2,
                Rarity::Epic => 5,
                Rarity::Legendary => 10,
            };

            total_value += value * item.weight;
        }

        total_value
    }

    pub fn find_by_name(&self, name: &str) -> Option<&Item> {
        // TODO (FR): renvoie une référence vers le premier item dont le nom correspond,
        // ou None s'il n'existe pas. Vise une seule expression avec un itérateur.
        self.items.iter().find(|&item| item.name == name)
    }

    pub fn weapons(&self) -> Vec<&Item> {
        // TODO (FR): renvoie un Vec contenant des références vers tous les items
        // dont le `kind` est une Weapon. Filtre et collecte.
        let result = self
            .items
            .iter()
            .filter(|&item| matches!(item.kind, ItemKind::Weapon { damage: _damage }))
            .collect();

        result
    }
}

impl IntoIterator for Backpack {
    type Item = Item;
    type IntoIter = std::vec::IntoIter<Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}

impl<'a> IntoIterator for &'a Backpack {
    type Item = &'a Item;
    type IntoIter = std::slice::Iter<'a, Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::item::ItemKind;

    fn sword() -> Item {
        Item {
            name: "Sword".to_string(),
            kind: ItemKind::Weapon { damage: 50 },
            rarity: Rarity::Epic,
            weight: 5,
        }
    }

    fn dagger() -> Item {
        Item {
            name: "Dagger".to_string(),
            kind: ItemKind::Weapon { damage: 15 },
            rarity: Rarity::Common,
            weight: 2,
        }
    }

    fn potion() -> Item {
        Item {
            name: "Health Potion".to_string(),
            kind: ItemKind::Potion { healing: 25 },
            rarity: Rarity::Common,
            weight: 1,
        }
    }

    fn shield() -> Item {
        Item {
            name: "Shield".to_string(),
            kind: ItemKind::Armor { defense: 30 },
            rarity: Rarity::Rare,
            weight: 10,
        }
    }

    #[test]
    fn new_backpack_is_empty() {
        let bag = Backpack::new(100);
        assert_eq!(bag.items.len(), 0);
        assert_eq!(bag.max_weight, 100);
    }

    #[test]
    fn add_item_succeeds_under_capacity() {
        let mut bag = Backpack::new(100);
        assert_eq!(bag.add_item(sword()), Ok(()));
        assert_eq!(bag.items.len(), 1);
    }

    #[test]
    fn add_item_fails_when_over_capacity() {
        let mut bag = Backpack::new(6);
        assert_eq!(bag.add_item(sword()), Ok(()));
        assert_eq!(bag.add_item(shield()), Err(InventoryError::BackpackFull));
        assert_eq!(bag.items.len(), 1);
    }

    #[test]
    fn remove_item_returns_the_item() {
        let mut bag = Backpack::new(100);
        bag.add_item(sword()).unwrap();
        bag.add_item(potion()).unwrap();
        let removed = bag.remove_item("Sword").unwrap();
        assert_eq!(removed, sword());
        assert_eq!(bag.items.len(), 1);
    }

    #[test]
    fn remove_unknown_item_returns_error() {
        let mut bag = Backpack::new(100);
        bag.add_item(potion()).unwrap();
        assert_eq!(bag.remove_item("Sword"), Err(InventoryError::ItemNotFound));
    }

    #[test]
    fn total_weight_sums_items() {
        let mut bag = Backpack::new(100);
        bag.add_item(sword()).unwrap();
        bag.add_item(potion()).unwrap();
        bag.add_item(shield()).unwrap();
        assert_eq!(bag.total_weight(), 16);
    }

    #[test]
    fn count_by_rarity_counts_correctly() {
        let mut bag = Backpack::new(100);
        bag.add_item(sword()).unwrap();
        bag.add_item(dagger()).unwrap();
        bag.add_item(potion()).unwrap();
        bag.add_item(shield()).unwrap();
        assert_eq!(bag.count_by_rarity(Rarity::Common), 2);
        assert_eq!(bag.count_by_rarity(Rarity::Rare), 1);
        assert_eq!(bag.count_by_rarity(Rarity::Epic), 1);
        assert_eq!(bag.count_by_rarity(Rarity::Legendary), 0);
    }

    #[test]
    fn strongest_weapon_returns_highest_damage() {
        let mut bag = Backpack::new(100);
        bag.add_item(dagger()).unwrap();
        bag.add_item(sword()).unwrap();
        bag.add_item(potion()).unwrap();
        let best = bag.strongest_weapon().unwrap();
        assert_eq!(best.name, "Sword");
    }

    #[test]
    fn strongest_weapon_returns_none_without_weapons() {
        let mut bag = Backpack::new(100);
        bag.add_item(potion()).unwrap();
        bag.add_item(shield()).unwrap();
        assert!(bag.strongest_weapon().is_none());
    }

    #[test]
    fn total_value_uses_rarity_multiplier() {
        let mut bag = Backpack::new(100);
        bag.add_item(sword()).unwrap();
        bag.add_item(potion()).unwrap();
        bag.add_item(shield()).unwrap();
        assert_eq!(bag.total_value(), 46);
    }

    #[test]
    fn find_by_name_returns_item_if_present() {
        let mut bag = Backpack::new(100);
        bag.add_item(sword()).unwrap();
        bag.add_item(potion()).unwrap();
        let found = bag.find_by_name("Sword").unwrap();
        assert_eq!(found.name, "Sword");
    }

    #[test]
    fn find_by_name_returns_none_if_absent() {
        let mut bag = Backpack::new(100);
        bag.add_item(potion()).unwrap();
        assert!(bag.find_by_name("Sword").is_none());
    }

    #[test]
    fn weapons_returns_only_weapons() {
        let mut bag = Backpack::new(100);
        bag.add_item(sword()).unwrap();
        bag.add_item(potion()).unwrap();
        bag.add_item(dagger()).unwrap();
        bag.add_item(shield()).unwrap();
        let weapons = bag.weapons();
        assert_eq!(weapons.len(), 2);
        assert!(weapons.iter().any(|w| w.name == "Sword"));
        assert!(weapons.iter().any(|w| w.name == "Dagger"));
    }

    #[test]
    fn borrowed_backpack_is_iterable() {
        let mut bag = Backpack::new(100);
        bag.add_item(sword()).unwrap();
        bag.add_item(potion()).unwrap();
        let mut count = 0;
        for _item in &bag {
            count += 1;
        }
        assert_eq!(count, 2);
        assert_eq!(bag.items.len(), 2);
    }

    #[test]
    fn owned_backpack_can_be_consumed_by_into_iter() {
        let mut bag = Backpack::new(100);
        bag.add_item(sword()).unwrap();
        bag.add_item(potion()).unwrap();
        let names: Vec<String> = bag.into_iter().map(|item| item.name).collect();
        assert_eq!(names.len(), 2);
        assert!(names.contains(&"Sword".to_string()));
        assert!(names.contains(&"Health Potion".to_string()));
    }
}
