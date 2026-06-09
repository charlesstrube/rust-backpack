use std::fmt::{Debug, Formatter, Result};

pub enum InventoryError {
    BackpackFull,
    ItemNotFound,
}

impl Debug for InventoryError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Self::BackpackFull => write!(f, "BackoackFull"),
            Self::ItemNotFound => write!(f, "ItemNotFound"),
        }
    }
}

impl PartialEq for InventoryError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::BackpackFull, Self::BackpackFull) => true,
            (Self::ItemNotFound, Self::ItemNotFound) => true,
            _ => false,
        }
    }
}
