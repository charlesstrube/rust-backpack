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

fn get_rarity_index(rarity: &Rarity) -> usize {
    match rarity {
        Rarity::Common => 0,
        Rarity::Rare => 1,
        Rarity::Epic => 2,
        Rarity::Legendary => 3,
    }
}

impl PartialOrd for Rarity {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Rarity {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_index = get_rarity_index(self);
        return self_index.cmp(&get_rarity_index(other));
    }
}
