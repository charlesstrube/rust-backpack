use crate::item::Item;

pub fn get_total_weight(items: &[Item]) -> u32 {
    items.iter().fold(0, |acc, item| acc + item.weight())
}
