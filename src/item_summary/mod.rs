use crate::item::{Item, item_kind::ItemKind};

#[allow(unused)]
pub struct ItemSummary {
    name: String,
    value: u32,
}

impl From<&Item> for ItemSummary {
    fn from(item: &Item) -> Self {
        match item.kind() {
            ItemKind::Armor { defense } => Self {
                name: item.name().into(),
                value: *defense,
            },
            ItemKind::Potion { healing } => Self {
                name: item.name().into(),
                value: *healing,
            },
            ItemKind::Weapon { damage } => Self {
                name: item.name().into(),
                value: *damage,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{item::item_kind::ItemKind, rarity::Rarity};

    fn sword() -> Item {
        Item::new("Sword", ItemKind::Weapon { damage: 50 }, Rarity::Epic, 5).unwrap()
    }
    #[test]
    fn item_summary_from_item_copies_name_and_value() {
        let s = sword();
        let summary: ItemSummary = (&s).into();
        assert_eq!(summary.name, "Sword");
        // adapt this assertion to your chosen value formula
        assert!(summary.value > 0);
    }
}
