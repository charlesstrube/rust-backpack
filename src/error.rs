use std::fmt::{Debug, Formatter};

use crate::item::{item_name::ItemNameError, item_weight::ItemWeightError};

pub enum InventoryError {
    BackpackFull,
    BackpackEmpty,
    ItemNotFound,
    WouldExceedCapacity,
    InvalidName,
    InvalidWeight,
    InvalidMaxWeight,
}

impl Debug for InventoryError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Self::BackpackFull => write!(f, "BackoackFull"),
            Self::ItemNotFound => write!(f, "ItemNotFound"),
            Self::WouldExceedCapacity => write!(f, "WouldExceedCapacity"),
            Self::InvalidName => write!(f, "InvalidName"),
            Self::BackpackEmpty => write!(f, "BackpackEmpty"),
            Self::InvalidWeight => write!(f, "InvalidWeight"),
            Self::InvalidMaxWeight => write!(f, "InvalidMaxWeight"),
        }
    }
}

impl From<ItemNameError> for InventoryError {
    fn from(value: ItemNameError) -> Self {
        match value {
            ItemNameError::NameIsEmpty => InventoryError::InvalidName,
        }
    }
}

impl From<ItemWeightError> for InventoryError {
    fn from(value: ItemWeightError) -> Self {
        match value {
            ItemWeightError::WeightIsZero => InventoryError::InvalidWeight,
        }
    }
}

impl PartialEq for InventoryError {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (Self::BackpackFull, Self::BackpackFull)
                | (Self::ItemNotFound, Self::ItemNotFound)
                | (Self::WouldExceedCapacity, Self::WouldExceedCapacity)
        )
    }
}

#[cfg(test)]
mod tests {
    // =====================================================================
    // PHASE 1.5 — Manual From<ItemNameError> for InventoryError
    // =====================================================================

    use super::*;
    use crate::item::item_name::{ItemName, ItemNameError};

    #[test]
    fn inventory_error_from_item_name_error_maps_to_invalid_name() {
        let src = ItemNameError::NameIsEmpty;
        let converted: InventoryError = src.into();
        // adapt the expected variant name to your enum
        assert!(matches!(converted, InventoryError::InvalidName));
    }

    #[test]
    fn question_mark_propagates_item_name_error_into_inventory_error() {
        fn make_name(raw: &str) -> Result<ItemName, InventoryError> {
            let name: ItemName = raw.try_into()?;
            Ok(name)
        }
        assert!(make_name("ok").is_ok());
        assert!(matches!(make_name(""), Err(InventoryError::InvalidName)));
    }

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
