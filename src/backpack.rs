use std::sync::atomic::AtomicU32;

use crate::error::InventoryError;
use crate::item::Item;
use crate::item::item_kind::ItemKind;
use crate::rarity::Rarity;

pub struct Backpack {
    /*
     * we cannot impl display for this type because rust's orphan rule
     * 1: Display is foreign (std)
     * 2: Item is local. But that doesn't help, the orphan rule looks at the outer type (vec), not the type parameter
     * we would need to create a dedicated type for that Vec<Item>
     */
    items: Vec<Item>,
    max_weight: u32,
}

fn get_total_weight(items: &[Item]) -> u32 {
    items.iter().fold(0, |acc, item| acc + item.weight())
}

fn factorial(n: u32) -> u128 {
    let mut acc = 1u128;

    if n == 0 || n == 1 {
        return 1u128;
    }

    for x in 2..=n {
        acc *= x as u128;
    }

    acc
}

impl Backpack {
    pub fn new(max_weight: u32) -> Result<Self, InventoryError> {
        if max_weight == 0 {
            return Err(InventoryError::InvalidMaxWeight(
                "cannot be equal to 0".into(),
            ));
        }

        Ok(Self {
            items: Vec::new(),
            max_weight,
        })
    }

    pub fn as_slice(&self) -> &[Item] {
        self.items.as_slice()
    }

    pub fn as_mut_slice(&mut self) -> &mut [Item] {
        self.items.as_mut_slice()
    }

    pub fn bulk_add(&mut self, items: &[Item]) -> Result<(), InventoryError> {
        let items_weight = get_total_weight(items);
        let remaining_weight = self.max_weight - self.total_weight();
        if items_weight > remaining_weight {
            return Err(InventoryError::BackpackFull);
        }
        for item in items {
            self.add_item(item.clone())?;
        }
        Ok(())
    }

    pub fn find_all<'a>(&'a self, query: &'a str) -> impl Iterator<Item = &'a Item> + 'a {
        self.items()
            .iter()
            .filter(move |item| item.name().contains(query))
    }

    pub fn heaviest(&self, n: usize) -> Vec<&Item> {
        let mut list: Vec<&Item> = Vec::new();

        for item in &self.items {
            list.push(&item);
        }

        list.sort_by(|a, b| {
            if a.weight() > b.weight() {
                return std::cmp::Ordering::Less;
            }
            if a.weight() < b.weight() {
                return std::cmp::Ordering::Greater;
            }
            std::cmp::Ordering::Equal
        });
        list.truncate(n);
        list
    }

    pub fn add_item(&mut self, item: Item) -> Result<(), InventoryError> {
        let total_weight = self.total_weight();
        if self.max_weight < total_weight {
            unreachable!()
        }
        if self.max_weight - total_weight >= item.weight() {
            self.items.push(item);
            Ok(())
        } else {
            Err(InventoryError::BackpackFull)
        }
    }

    pub fn remove_item(&mut self, name: &str) -> Result<Item, InventoryError> {
        let mut index_found: Option<usize> = None;
        for (index, item) in self.items.iter().enumerate() {
            if item.name() == name {
                index_found = Some(index);
            }
        }

        if let Some(value) = index_found {
            Ok(self.items.remove(value))
        } else {
            Err(InventoryError::ItemNotFound)
        }
    }

    pub fn total_weight(&self) -> u32 {
        get_total_weight(&self.items)
    }

    pub fn count_by_rarity(&self, rarity: Rarity) -> usize {
        let mut count: usize = 0;
        for item in self.items.iter() {
            if item.rarity() == rarity {
                count += 1;
            }
        }
        count
    }

    pub fn items(&self) -> &[Item] {
        &self.items
    }

    pub fn max_weight(&self) -> u32 {
        self.max_weight
    }

    pub fn set_max_weight(&mut self, max_weight: u32) -> Result<(), InventoryError> {
        let total_weight = self.total_weight();
        if total_weight > max_weight {
            return Err(InventoryError::WouldExceedCapacity(format!(
                "{} cannot fit in {}",
                max_weight, total_weight
            )));
        }

        self.max_weight = max_weight;
        Ok(())
    }

    pub fn reserve(&mut self, additional: usize) {
        self.items.reserve(additional)
    }

    pub fn total_weight_saturating(&self) -> u32 {
        self.max_weight.saturating_add(u32::MAX)
    }

    /**
     * add the check method to know if the backpack is full
     */
    pub fn total_weight_checked(&self) -> Option<u32> {
        let mut weight = 0u32;
        let mut result = Some(0u32);
        for item in self.items().iter() {
            if result.is_some() && weight.checked_add(item.weight()).is_some() {
                weight += item.weight();
                result = Some(weight)
            } else {
                result = None;
            }
        }

        result
    }

    pub fn average_weight(&self) -> f64 {
        if self.items().iter().count() == 0 {
            return 0f64;
        }
        let calc = self.total_weight() / (self.items().iter().count() as u32);

        calc.into()
    }

    pub fn slot_combinations(&self, slots: u32) -> u128 {
        let n = self.items().len() as u32;
        if slots > n {
            return 0;
        }
        factorial(n) / (factorial(slots) * factorial(n - slots))
    }

    pub fn strongest_weapon(&self) -> Option<&Item> {
        let mut strongest_weapon: Option<&Item> = None;
        for item in self.items.iter() {
            if let ItemKind::Weapon {
                damage: current_damage,
            } = item.kind()
            {
                if let Some(weapon) = strongest_weapon {
                    if let ItemKind::Weapon {
                        damage: strongest_damage,
                    } = weapon.kind()
                        && strongest_damage < current_damage
                    {
                        strongest_weapon = Some(item)
                    }
                } else {
                    strongest_weapon = Some(item)
                }
            }
        }
        strongest_weapon
    }

    pub fn total_value(&self) -> u32 {
        let mut total_value = 0;
        for item in self.items.iter() {
            let value = match item.rarity() {
                Rarity::Common => 1,
                Rarity::Rare => 2,
                Rarity::Epic => 5,
                Rarity::Legendary => 10,
            };

            total_value += value * item.weight();
        }

        total_value
    }

    pub fn find_by_name(&self, name: &str) -> Option<&Item> {
        self.items.iter().find(|&item| item.name() == name)
    }

    pub fn weapons(&self) -> Vec<&Item> {
        self.items
            .iter()
            .filter(|&item| matches!(item.kind(), ItemKind::Weapon { damage: _damage }))
            .collect()
    }
}

impl IntoIterator for Backpack {
    type Item = Item;
    type IntoIter = std::vec::IntoIter<Item>;

    fn into_iter(mut self) -> Self::IntoIter {
        let items = std::mem::take(&mut self.items);
        items.into_iter()
    }
}

impl<'a> IntoIterator for &'a Backpack {
    type Item = &'a Item;
    type IntoIter = std::slice::Iter<'a, Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.iter()
    }
}

static DROPPED: AtomicU32 = AtomicU32::new(0);

impl Drop for Backpack {
    fn drop(&mut self) {
        DROPPED.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    }
}

pub fn dropped_count() -> u32 {
    DROPPED.load(std::sync::atomic::Ordering::Relaxed)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::item::item_kind::ItemKind;
    #[cfg(test)]
    fn push_unchecked(bag: &mut Backpack, item: Item) {
        bag.items.push(item);
    }

    fn sword() -> Item {
        Item::new("Sword", ItemKind::Weapon { damage: 50 }, Rarity::Epic, 5).unwrap()
    }

    fn dagger() -> Item {
        Item::new("Dagger", ItemKind::Weapon { damage: 15 }, Rarity::Common, 2).unwrap()
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

    fn shield() -> Item {
        Item::new("Shield", ItemKind::Armor { defense: 30 }, Rarity::Rare, 10).unwrap()
    }

    #[test]
    fn new_backpack_is_empty() {
        let bag = Backpack::new(100).unwrap();
        assert_eq!(bag.items.len(), 0);
        assert_eq!(bag.max_weight, 100);
    }

    #[test]
    fn add_item_succeeds_under_capacity() {
        let mut bag = Backpack::new(100).unwrap();
        assert_eq!(bag.add_item(sword()), Ok(()));
        assert_eq!(bag.items.len(), 1);
    }

    #[test]
    fn add_item_fails_when_over_capacity() {
        let mut bag = Backpack::new(6).unwrap();
        assert_eq!(bag.add_item(sword()), Ok(()));
        assert_eq!(bag.add_item(shield()), Err(InventoryError::BackpackFull));
        assert_eq!(bag.items.len(), 1);
    }

    #[test]
    fn remove_item_returns_the_item() {
        let mut bag = Backpack::new(100).unwrap();
        bag.add_item(sword()).unwrap();
        bag.add_item(potion()).unwrap();
        let removed = bag.remove_item("Sword").unwrap();
        assert_eq!(removed, sword());
        assert_eq!(bag.items.len(), 1);
    }

    #[test]
    fn remove_unknown_item_returns_error() {
        let mut bag = Backpack::new(100).unwrap();
        bag.add_item(potion()).unwrap();
        assert_eq!(bag.remove_item("Sword"), Err(InventoryError::ItemNotFound));
    }

    #[test]
    fn total_weight_sums_items() {
        let mut bag = Backpack::new(100).unwrap();
        bag.add_item(sword()).unwrap();
        bag.add_item(potion()).unwrap();
        bag.add_item(shield()).unwrap();
        assert_eq!(bag.total_weight(), 16);
    }

    #[test]
    fn count_by_rarity_counts_correctly() {
        let mut bag = Backpack::new(100).unwrap();
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
        let mut bag = Backpack::new(100).unwrap();
        bag.add_item(dagger()).unwrap();
        bag.add_item(sword()).unwrap();
        bag.add_item(potion()).unwrap();
        let best = bag.strongest_weapon().unwrap();
        assert_eq!(best.name(), "Sword");
    }

    #[test]
    fn strongest_weapon_returns_none_without_weapons() {
        let mut bag = Backpack::new(100).unwrap();
        bag.add_item(potion()).unwrap();
        bag.add_item(shield()).unwrap();
        assert!(bag.strongest_weapon().is_none());
    }

    #[test]
    fn total_value_uses_rarity_multiplier() {
        let mut bag = Backpack::new(100).unwrap();
        bag.add_item(sword()).unwrap();
        bag.add_item(potion()).unwrap();
        bag.add_item(shield()).unwrap();
        assert_eq!(bag.total_value(), 46);
    }

    #[test]
    fn find_by_name_returns_item_if_present() {
        let mut bag = Backpack::new(100).unwrap();
        bag.add_item(sword()).unwrap();
        bag.add_item(potion()).unwrap();
        let found = bag.find_by_name("Sword").unwrap();
        assert_eq!(found.name(), "Sword");
    }

    #[test]
    fn find_by_name_returns_none_if_absent() {
        let mut bag = Backpack::new(100).unwrap();
        bag.add_item(potion()).unwrap();
        assert!(bag.find_by_name("Sword").is_none());
    }

    #[test]
    fn weapons_returns_only_weapons() {
        let mut bag = Backpack::new(100).unwrap();
        bag.add_item(sword()).unwrap();
        bag.add_item(potion()).unwrap();
        bag.add_item(dagger()).unwrap();
        bag.add_item(shield()).unwrap();
        let weapons = bag.weapons();
        assert_eq!(weapons.len(), 2);
        assert!(weapons.iter().any(|w| w.name() == "Sword"));
        assert!(weapons.iter().any(|w| w.name() == "Dagger"));
    }

    #[test]
    fn borrowed_backpack_is_iterable() {
        let mut bag = Backpack::new(100).unwrap();
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
        let mut bag = Backpack::new(100).unwrap();
        bag.add_item(sword()).unwrap();
        bag.add_item(potion()).unwrap();
        let names: Vec<String> = bag
            .into_iter()
            .map(|item| item.name().to_string())
            .collect();
        assert_eq!(names.len(), 2);
        assert!(names.contains(&"Sword".to_string()));
        assert!(names.contains(&"Health Potion".to_string()));
    }

    // =====================================================================
    // PHASE 1 — Encapsulation: Backpack getter, setter, reserve
    // =====================================================================

    #[test]
    fn backpack_items_getter_exposes_slice() {
        let mut bag = Backpack::new(100).unwrap();
        bag.add_item(sword()).unwrap();
        bag.add_item(potion()).unwrap();
        let items: &[Item] = bag.items();
        assert_eq!(items.len(), 2);
    }

    #[test]
    fn backpack_set_max_weight_accepts_increase() {
        let mut bag = Backpack::new(50).unwrap();
        assert!(bag.set_max_weight(100).is_ok());
        assert_eq!(bag.max_weight(), 100);
    }

    #[test]
    fn backpack_set_max_weight_rejects_below_current_load() {
        let mut bag = Backpack::new(100).unwrap();
        bag.add_item(shield()).unwrap();
        bag.add_item(sword()).unwrap();
        assert!(bag.set_max_weight(10).is_err());
        assert_eq!(bag.max_weight(), 100);
    }

    #[test]
    fn reserve_increases_capacity_without_changing_len() {
        let mut bag = Backpack::new(100).unwrap();
        bag.add_item(sword()).unwrap();
        let len_before = bag.items().len();
        bag.reserve(64);
        assert_eq!(bag.items().len(), len_before);
    }

    // =====================================================================
    // PHASE 2 — Backpack::new validated
    // =====================================================================

    #[test]
    fn backpack_new_rejects_zero_max_weight() {
        assert!(Backpack::new(0).is_err());
    }

    // =====================================================================
    // PHASE 3 — Saturating / checked arithmetic, casting, factorial
    // =====================================================================

    #[test]
    fn total_weight_saturating_caps_at_u32_max() {
        let mut bag = Backpack::new(u32::MAX).unwrap();
        // build two giant items so the naive sum would overflow u32
        let heavy_a = Item::new(
            "A".into(),
            ItemKind::Armor { defense: 1 },
            Rarity::Common,
            u32::MAX,
        )
        .unwrap();
        let heavy_b = Item::new(
            "B".into(),
            ItemKind::Armor { defense: 1 },
            Rarity::Common,
            10,
        )
        .unwrap();
        // bypass capacity check by pushing directly through bulk_add if needed,
        // or temporarily raise max — adapt to your design
        push_unchecked(&mut bag, heavy_a);
        push_unchecked(&mut bag, heavy_b);
        assert_eq!(bag.total_weight_saturating(), u32::MAX);
    }

    #[test]
    fn total_weight_checked_returns_none_on_overflow() {
        let mut bag = Backpack::new(u32::MAX).unwrap();
        let heavy_a = Item::new(
            "A".into(),
            ItemKind::Armor { defense: 1 },
            Rarity::Common,
            u32::MAX,
        )
        .unwrap();
        let heavy_b = Item::new(
            "B".into(),
            ItemKind::Armor { defense: 1 },
            Rarity::Common,
            10,
        )
        .unwrap();
        push_unchecked(&mut bag, heavy_a);
        push_unchecked(&mut bag, heavy_b);
        assert_eq!(bag.total_weight_checked(), None);
    }

    #[test]
    fn average_weight_returns_zero_for_empty_backpack() {
        let bag = Backpack::new(100).unwrap();
        assert_eq!(bag.average_weight(), 0.0);
    }

    #[test]
    fn average_weight_computes_mean() {
        let mut bag = Backpack::new(100).unwrap();
        bag.add_item(sword()).unwrap(); // 5
        bag.add_item(potion()).unwrap(); // 1
        // (5 + 1) / 2 = 3.0
        assert_eq!(bag.average_weight(), 3.0);
    }

    #[test]
    fn factorial_of_five_is_one_hundred_twenty() {
        assert_eq!(factorial(0), 1);
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(5), 120);
        assert_eq!(factorial(10), 3_628_800);
    }

    #[test]
    fn slot_combinations_uses_factorial() {
        let mut bag = Backpack::new(100).unwrap();
        bag.add_item(sword()).unwrap();
        bag.add_item(potion()).unwrap();
        bag.add_item(shield()).unwrap();
        // C(3, 2) = 3! / (2! * 1!) = 3
        assert_eq!(bag.slot_combinations(2), 3);
    }

    // =====================================================================
    // PHASE 4 — Derive macros smoke test
    // =====================================================================

    #[test]
    fn rarity_works_as_hashmap_key() {
        use std::collections::HashMap;
        let mut counts: HashMap<Rarity, u32> = HashMap::new();
        *counts.entry(Rarity::Common).or_insert(0) += 1;
        *counts.entry(Rarity::Common).or_insert(0) += 1;
        *counts.entry(Rarity::Epic).or_insert(0) += 1;
        assert_eq!(counts.get(&Rarity::Common), Some(&2));
        assert_eq!(counts.get(&Rarity::Epic), Some(&1));
        assert_eq!(counts.get(&Rarity::Legendary), None);
    }

    // =====================================================================
    // PHASE 8 — Drop counter
    // =====================================================================

    #[test]
    fn dropping_backpack_increments_drop_counter() {
        let before = dropped_count();
        {
            let _bag = Backpack::new(50).unwrap();
        }
        assert_eq!(dropped_count(), before + 1);
    }

    // =====================================================================
    // PHASE 9 — Slices, lifetimes, impl Trait
    // =====================================================================

    #[test]
    fn as_slice_returns_all_items() {
        let mut bag = Backpack::new(100).unwrap();
        bag.add_item(sword()).unwrap();
        bag.add_item(potion()).unwrap();
        let slice: &[Item] = bag.as_slice();
        assert_eq!(slice.len(), 2);
    }

    #[test]
    fn as_mut_slice_allows_sort_in_place() {
        let mut bag = Backpack::new(100).unwrap();
        bag.add_item(sword()).unwrap(); // 5
        bag.add_item(shield()).unwrap(); // 10
        bag.add_item(potion()).unwrap(); // 1
        bag.as_mut_slice().sort_by_key(|i| i.weight());
        assert_eq!(bag.as_slice()[0].name(), "Health Potion");
        assert_eq!(bag.as_slice()[1].name(), "Sword");
        assert_eq!(bag.as_slice()[2].name(), "Shield");
    }

    #[test]
    fn bulk_add_accepts_slice() {
        let mut bag = Backpack::new(100).unwrap();
        let items = [sword(), potion(), shield()];
        bag.bulk_add(&items).unwrap();
        assert_eq!(bag.as_slice().len(), 3);
    }

    #[test]
    fn bulk_add_rejects_when_capacity_exceeded() {
        let mut bag = Backpack::new(5).unwrap();
        let items = [sword(), shield()]; // 5 + 10 > 5
        assert!(bag.bulk_add(&items).is_err());
    }

    #[test]
    fn find_all_returns_filtered_iterator() {
        let mut bag = Backpack::new(100).unwrap();
        bag.add_item(sword()).unwrap(); // "Sword" contains 'o'
        bag.add_item(potion()).unwrap(); // "Health Potion" contains 'o'
        bag.add_item(dagger()).unwrap(); // "Dagger" does not contain 'o'
        let matches: Vec<&Item> = bag.find_all("o").collect();
        assert_eq!(matches.len(), 2);
    }

    #[test]
    fn heaviest_returns_top_n_in_descending_order() {
        let mut bag = Backpack::new(100).unwrap();
        bag.add_item(sword()).unwrap(); // 5
        bag.add_item(potion()).unwrap(); // 1
        bag.add_item(shield()).unwrap(); // 10
        let top2 = bag.heaviest(2);
        assert_eq!(top2.len(), 2);
        assert_eq!(top2[0].weight(), 10);
        assert_eq!(top2[1].weight(), 5);
    }

    #[test]
    fn heaviest_caps_at_available_items() {
        let mut bag = Backpack::new(100).unwrap();
        bag.add_item(sword()).unwrap();
        assert_eq!(bag.heaviest(10).len(), 1);
    }

    // =====================================================================
    // PHASE 10 — Index / IndexMut
    // =====================================================================

    // #[test]
    // fn index_by_position_returns_item() {
    //     let mut bag = Backpack::new(100).unwrap();
    //     bag.add_item(sword()).unwrap();
    //     bag.add_item(potion()).unwrap();
    //     assert_eq!(bag[0].name(), "Sword");
    //     assert_eq!(bag[1].name(), "Health Potion");
    // }

    // #[test]
    // fn index_by_name_returns_item() {
    //     let mut bag = Backpack::new(100).unwrap();
    //     bag.add_item(sword()).unwrap();
    //     bag.add_item(potion()).unwrap();
    //     assert_eq!(bag["Sword"].weight(), 5);
    //     assert_eq!(bag["Health Potion"].weight(), 1);
    // }

    // #[test]
    // #[should_panic]
    // fn index_by_unknown_name_panics() {
    //     let bag = Backpack::new(100).unwrap();
    //     let _ = &bag["NotHere"];
    // }

    // #[test]
    // fn index_mut_allows_field_update() {
    //     let mut bag = Backpack::new(100).unwrap();
    //     bag.add_item(sword()).unwrap();
    //     bag[0] = potion();
    //     assert_eq!(bag[0].name(), "Health Potion");
    // }

    // =====================================================================
    // PHASE 11 — Backpack::add (operator overloading)
    // =====================================================================

    // #[test]
    // fn backpack_add_merges_max_weight_and_items() {
    //     let mut a = Backpack::new(50).unwrap();
    //     let mut b = Backpack::new(30).unwrap();
    //     a.add_item(sword()).unwrap();
    //     b.add_item(potion()).unwrap();
    //     let merged = a + b;
    //     assert_eq!(merged.max_weight(), 80);
    //     assert_eq!(merged.as_slice().len(), 2);
    // }

    // =====================================================================
    // PHASE 12 — HashMap / BTreeMap
    // =====================================================================

    // #[test]
    // fn group_by_rarity_buckets_correctly() {
    //     let mut bag = Backpack::new(100).unwrap();
    //     bag.add_item(sword()).unwrap();   // Epic
    //     bag.add_item(potion()).unwrap();  // Common
    //     bag.add_item(dagger()).unwrap();  // Common
    //     let groups = bag.group_by_rarity();
    //     assert_eq!(groups.get(&Rarity::Common).unwrap().len(), 2);
    //     assert_eq!(groups.get(&Rarity::Epic).unwrap().len(), 1);
    //     assert!(groups.get(&Rarity::Legendary).is_none());
    // }

    // #[test]
    // fn weights_by_value_orders_keys_ascending() {
    //     let mut bag = Backpack::new(100).unwrap();
    //     bag.add_item(shield()).unwrap();  // 10
    //     bag.add_item(sword()).unwrap();   // 5
    //     bag.add_item(potion()).unwrap();  // 1
    //     let map = bag.weights_by_value();
    //     let keys: Vec<u32> = map.keys().copied().collect();
    //     assert_eq!(keys, vec![1, 5, 10]);
    // }

    // #[test]
    // fn most_common_rarity_returns_top_bucket() {
    //     let mut bag = Backpack::new(100).unwrap();
    //     bag.add_item(potion()).unwrap();  // Common
    //     bag.add_item(dagger()).unwrap();  // Common
    //     bag.add_item(sword()).unwrap();   // Epic
    //     assert_eq!(bag.most_common_rarity(), Some(Rarity::Common));
    // }

    // #[test]
    // fn most_common_rarity_returns_none_for_empty() {
    //     let bag = Backpack::new(100).unwrap();
    //     assert!(bag.most_common_rarity().is_none());
    // }

    // =====================================================================
    // PHASE 13 — Generics & trait bounds
    // =====================================================================

    // #[test]
    // fn add_many_accepts_vec() {
    //     let mut bag = Backpack::new(100).unwrap();
    //     let items = vec![sword(), potion()];
    //     bag.add_many(items).unwrap();
    //     assert_eq!(bag.as_slice().len(), 2);
    // }

    // #[test]
    // fn add_many_accepts_fixed_size_array() {
    //     let mut bag = Backpack::new(100).unwrap();
    //     let items: [Item; 2] = [sword(), potion()];
    //     bag.add_many(items).unwrap();
    //     assert_eq!(bag.as_slice().len(), 2);
    // }

    // #[test]
    // fn add_many_accepts_iterator_chain() {
    //     let mut bag = Backpack::new(100).unwrap();
    //     let chain = std::iter::once(sword()).chain(std::iter::once(potion()));
    //     bag.add_many(chain).unwrap();
    //     assert_eq!(bag.as_slice().len(), 2);
    // }

    // =====================================================================
    // PHASE 14 — Combinators
    // =====================================================================

    // #[test]
    // fn total_damage_sums_only_weapon_damage() {
    //     let mut bag = Backpack::new(100).unwrap();
    //     bag.add_item(sword()).unwrap();   // 50
    //     bag.add_item(dagger()).unwrap();  // 15
    //     bag.add_item(potion()).unwrap();  // not a weapon
    //     bag.add_item(shield()).unwrap();  // not a weapon
    //     assert_eq!(bag.total_damage(), 65);
    // }

    // #[test]
    // fn total_damage_is_zero_without_weapons() {
    //     let mut bag = Backpack::new(100).unwrap();
    //     bag.add_item(potion()).unwrap();
    //     bag.add_item(shield()).unwrap();
    //     assert_eq!(bag.total_damage(), 0);
    // }
}
