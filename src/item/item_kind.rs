use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum ParseItemKindError {
    #[error("invalid kind: {0}")]
    InvalidKind(String),
    #[error("invalid unit: {0}")]
    InvalidUnit(String),
    #[error("missing semi col: {0}")]
    MissingSemiColon(String),
    #[error("too manu semi col: {0}")]
    TooMuchSemiColon(String),
}

#[derive(Debug, PartialEq, Clone)]
pub enum ItemKind {
    Weapon { damage: u32 },
    Potion { healing: u32 },
    Armor { defense: u32 },
}

impl Display for ItemKind {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Self::Weapon { damage } => write!(f, "Weapon with damage {}", damage),
            Self::Potion { healing } => write!(f, "Potion with healing {}", healing),
            Self::Armor { defense } => write!(f, "Armor with defense {}", defense),
        }
    }
}

impl TryFrom<&str> for ItemKind {
    type Error = ParseItemKindError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let lowercase = value.to_lowercase();
        let (kind, unit) = match lowercase.split_once(":") {
            Some((kind, unit)) => (kind, unit),
            None => {
                return Err(ParseItemKindError::MissingSemiColon(value.into()));
            }
        };

        let unit = match unit.trim().parse::<u32>() {
            Ok(result) => result,
            Err(_) => return Err(ParseItemKindError::InvalidUnit(unit.into())),
        };

        match kind {
            "weapon" => Ok(Self::Weapon { damage: unit }),
            "potion" => Ok(Self::Potion { healing: unit }),
            "armor" => Ok(Self::Armor { defense: unit }),
            _ => Err(ParseItemKindError::InvalidKind(kind.into())),
        }
    }
}

impl TryFrom<String> for ItemKind {
    type Error = ParseItemKindError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

// impl Debug for ItemKind {
//     fn fmt(&self, f: &mut Formatter) -> Result {
//         match self {
//             Self::Weapon { damage } => write!(f, "Weapon: {{ damage: {} }} ", damage),
//             Self::Potion { healing } => write!(f, "Potion {{ healing: {} }}", healing),
//             Self::Armor { defense } => write!(f, "Armor {{ defense: {} }}", defense),
//         }
//     }
// }

// impl PartialEq for ItemKind {
//     fn eq(&self, other: &Self) -> bool {
//         match (self, other) {
//             (
//                 Self::Armor {
//                     defense: self_defense,
//                 },
//                 Self::Armor {
//                     defense: other_defense,
//                 },
//             ) => self_defense == other_defense,
//             (
//                 Self::Potion {
//                     healing: self_healing,
//                 },
//                 Self::Potion {
//                     healing: other_healing,
//                 },
//             ) => self_healing == other_healing,
//             (
//                 Self::Weapon {
//                     damage: self_damage,
//                 },
//                 Self::Weapon {
//                     damage: other_damage,
//                 },
//             ) => self_damage == other_damage,
//             _ => false,
//         }
//     }
// }
