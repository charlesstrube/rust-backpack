use std::fmt::Debug;

use crate::{
    item::{item_kind::ParseItemKindError, item_weight::ItemWeightError},
    rarity::ParseRarityError,
};

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum InventoryError {
    #[error("The backpack is full")]
    BackpackFull,
    #[error("Invalid max weight: {0}")]
    InvalidMaxWeight(String),
    #[error("Item not found in the backpack")]
    ItemNotFound,
    #[error("Item cannot fit in the backpack: {0}")]
    WouldExceedCapacity(String),
    #[error("Ivalid name: {0}")]
    InvalidName(String),
    #[error("Invalid item weight: {0}")]
    InvalidWeight(String),
    #[error("Rarity parsing error")]
    ParseRarity(#[from] ParseRarityError),
    #[error("Item kind parsing error")]
    ParseKind(#[from] ParseItemKindError),
}

impl From<ItemWeightError> for InventoryError {
    fn from(value: ItemWeightError) -> Self {
        match value {
            ItemWeightError::WeightIsZero => {
                InventoryError::InvalidWeight("cannot be equal to 0".into())
            }
        }
    }
}

// impl PartialEq for InventoryError {
//     fn eq(&self, other: &Self) -> bool {
//         matches!(
//             (self, other),
//             (Self::BackpackFull, Self::BackpackFull)
//                 | (Self::InvalidMaxWeight, Self::InvalidMaxWeight)
//                 | (Self::ItemNotFound, Self::ItemNotFound)
//                 | (Self::WouldExceedCapacity, Self::WouldExceedCapacity)
//                 | (Self::InvalidName, Self::InvalidName)
//                 | (Self::InvalidWeight, Self::InvalidWeight)
//         )
//     }
// }

#[cfg(test)]
mod tests {

    // =====================================================================
    // PHASE 5 — thiserror, std::error::Error, source chaining
    // =====================================================================

    use std::error::Error;

    use crate::{error::InventoryError, rarity::Rarity};

    #[test]
    fn inventory_error_implements_std_error() {
        fn assert_error<E: Error>(_: &E) {}
        let err = InventoryError::BackpackFull;
        assert_error(&err);
    }

    #[test]
    fn inventory_error_display_shows_human_message() {
        let err = InventoryError::ItemNotFound;
        let rendered = format!("{}", err);
        assert!(!rendered.is_empty());
        assert!(rendered.to_lowercase().contains("not found"));
    }

    #[test]
    fn inventory_error_display_for_backpack_full() {
        let err = InventoryError::BackpackFull;
        let rendered = format!("{}", err);
        assert!(rendered.to_lowercase().contains("full"));
    }

    #[test]
    fn inventory_error_source_returns_inner_for_parse_variant() {
        // requires Phase 6: ParseRarityError + #[from] on the Parse variant
        let parse_err = Rarity::try_from("mythic").unwrap_err();
        let err: InventoryError = parse_err.into();
        assert!(err.source().is_some());
    }

    #[test]
    fn inventory_error_source_is_none_for_leaf_variants() {
        let err = InventoryError::BackpackFull;
        assert!(err.source().is_none());
    }
}
