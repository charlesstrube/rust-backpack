use std::fmt::{Debug, Display, Formatter, Result};

pub enum ItemKind {
    Weapon { damage: u32 },
    Potion { healing: u32 },
    Armor { defense: u32 },
}

impl Debug for ItemKind {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Self::Weapon { damage } => write!(f, "Weapon: {{ damage: {} }} ", damage),
            Self::Potion { healing } => write!(f, "Potion {{ healing: {} }}", healing),
            Self::Armor { defense } => write!(f, "Armor {{ defense: {} }}", defense),
        }
    }
}

impl Display for ItemKind {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Self::Weapon { damage } => write!(f, "Weapon with damage {}", damage),
            Self::Potion { healing } => write!(f, "Potion with healing {}", healing),
            Self::Armor { defense } => write!(f, "Armor with defense {}", defense),
        }
    }
}

impl PartialEq for ItemKind {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                Self::Armor {
                    defense: self_defense,
                },
                Self::Armor {
                    defense: other_defense,
                },
            ) => self_defense == other_defense,
            (
                Self::Potion {
                    healing: self_healing,
                },
                Self::Potion {
                    healing: other_healing,
                },
            ) => self_healing == other_healing,
            (
                Self::Weapon {
                    damage: self_damage,
                },
                Self::Weapon {
                    damage: other_damage,
                },
            ) => self_damage == other_damage,
            _ => false,
        }
    }
}
