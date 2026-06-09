use std::fmt::{Debug, Formatter, Result};

pub enum InventoryError {
    BackpackFull,
    ItemNotFound,
    NotEnoughWeight,
}

impl Debug for InventoryError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Self::BackpackFull => write!(f, "BackoackFull"),
            Self::ItemNotFound => write!(f, "ItemNotFound"),
            Self::NotEnoughWeight => write!(f, "NotEnoughWeight"),
        }
    }
}

impl PartialEq for InventoryError {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (Self::BackpackFull, Self::BackpackFull)
                | (Self::ItemNotFound, Self::ItemNotFound)
                | (Self::NotEnoughWeight, Self::NotEnoughWeight)
        )
    }
}

#[cfg(test)]
mod tests {
    // =====================================================================
    // PHASE 5 — thiserror, std::error::Error, source chaining
    // =====================================================================

    // use super::*;
    // use crate::item::Rarity;
    // use std::error::Error;

    // #[test]
    // fn inventory_error_implements_std_error() {
    //     fn assert_error<E: Error>(_: &E) {}
    //     let err = InventoryError::BackpackFull;
    //     assert_error(&err);
    // }

    // #[test]
    // fn inventory_error_display_shows_human_message() {
    //     let err = InventoryError::ItemNotFound;
    //     let rendered = format!("{}", err);
    //     assert!(!rendered.is_empty());
    //     assert!(rendered.to_lowercase().contains("not found"));
    // }

    // #[test]
    // fn inventory_error_display_for_backpack_full() {
    //     let err = InventoryError::BackpackFull;
    //     let rendered = format!("{}", err);
    //     assert!(rendered.to_lowercase().contains("full"));
    // }

    // #[test]
    // fn inventory_error_source_returns_inner_for_parse_variant() {
    //     // requires Phase 6: ParseRarityError + #[from] on the Parse variant
    //     let parse_err = Rarity::try_from("mythic").unwrap_err();
    //     let err: InventoryError = parse_err.into();
    //     assert!(err.source().is_some());
    // }

    // #[test]
    // fn inventory_error_source_is_none_for_leaf_variants() {
    //     let err = InventoryError::BackpackFull;
    //     assert!(err.source().is_none());
    // }
}
