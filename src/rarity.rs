use std::fmt::Debug;

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum ParseRarityError {
    #[error("unknown rarity value: {0}")]
    UnknownValue(String),
}

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum Rarity {
    Common,
    Rare,
    Epic,
    Legendary,
}

impl TryFrom<&str> for Rarity {
    type Error = ParseRarityError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_string())
    }
}

impl TryFrom<String> for Rarity {
    type Error = ParseRarityError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "common" => Ok(Self::Common),
            "rare" => Ok(Self::Rare),
            "epic" => Ok(Self::Epic),
            "legendary" => Ok(Self::Legendary),
            _ => Err(ParseRarityError::UnknownValue(value)),
        }
    }
}
