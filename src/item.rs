use std::fmt::{Debug, Display, Formatter, Result};

pub enum Rarity {
    Common,
    Rare,
    Epic,
    Legendary,
}

impl Debug for Rarity {
    fn fmt(&self, f: &mut Formatter) -> Result {
        // TODO (FR): écris le nom de la variante dans le formatter.
        match self {
            Rarity::Common => write!(f, "Common"),
            Rarity::Rare => write!(f, "Rare"),
            Rarity::Epic => write!(f, "Epic"),
            Rarity::Legendary => write!(f, "Legendary"),
        }
    }
}

impl PartialEq for Rarity {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Rarity::Common, Rarity::Common) => true,
            (Rarity::Rare, Rarity::Rare) => true,
            (Rarity::Epic, Rarity::Epic) => true,
            (Rarity::Legendary, Rarity::Legendary) => true,
            __ => false,
        }
    }
}

// TODO (FR): Copy est un trait sans méthode. Il signale à Rust que ce type
// peut être dupliqué par copie de bits. Copy nécessite que Clone soit implémenté.
impl Copy for Rarity {}

impl Clone for Rarity {
    fn clone(&self) -> Self {
        // TODO (FR): renvoie une copie de self.
        match self {
            Rarity::Common => Rarity::Common,
            Rarity::Rare => Rarity::Rare,
            Rarity::Epic => Rarity::Epic,
            Rarity::Legendary => Rarity::Legendary,
        }
    }
}

pub enum ItemKind {
    Weapon { damage: u32 },
    Potion { healing: u32 },
    Armor { defense: u32 },
}

impl Debug for ItemKind {
    fn fmt(&self, f: &mut Formatter) -> Result {
        // TODO (FR): écris le nom de la variante + sa donnée associée dans le formatter.
        match self {
            Self::Weapon { damage } => write!(f, "Weapon: {{ damage: {} }} ", damage),
            Self::Potion { healing } => write!(f, "Potion {{ healing: {} }}", healing),
            Self::Armor { defense } => write!(f, "Armor {{ defense: {} }}", defense),
        }
    }
}

impl Display for ItemKind {
    fn fmt(&self, f: &mut Formatter) -> Result {
        // TODO (FR): écris le nom de la variante + sa donnée associée dans le formatter.
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

pub struct Item {
    pub name: String,
    pub kind: ItemKind,
    pub rarity: Rarity,
    pub weight: u32,
}

impl Debug for Item {
    fn fmt(&self, f: &mut Formatter) -> Result {
        // TODO (FR): écris une représentation lisible de l'Item avec tous ses champs.
        write!(
            f,
            "Item {{ name: {:?}, kind: {:?}, rarity: {:?}, weight: {:?} }}",
            self.name, self.kind, self.rarity, self.weight,
        )
    }
}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.kind == other.kind
            && self.rarity == other.rarity
            && self.weight == other.weight
    }
}

trait Describable {
    fn describe(&self) -> String;
}

impl Describable for Item {
    fn describe(&self) -> String {
        // TODO (FR): renvoie une String qui contient le nom, la rareté,
        // le type d'item (Weapon/Potion/Armor), la valeur associée (damage/healing/defense)
        // et le poids.

        format!(
            "{} is item with {} type and a weight of {}, and has a {:?} rarity",
            self.name, self.kind, self.weight, self.rarity
        )
        .into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sword() -> Item {
        Item {
            name: "Sword".to_string(),
            kind: ItemKind::Weapon { damage: 50 },
            rarity: Rarity::Epic,
            weight: 5,
        }
    }

    fn potion() -> Item {
        Item {
            name: "Health Potion".to_string(),
            kind: ItemKind::Potion { healing: 25 },
            rarity: Rarity::Common,
            weight: 1,
        }
    }

    #[test]
    fn describe_weapon_contains_expected_fields() {
        let s = sword().describe();
        assert!(s.contains("Sword"));
        assert!(s.contains("Epic"));
        assert!(s.contains("Weapon"));
        assert!(s.contains("50"));
        assert!(s.contains("5"));
    }

    #[test]
    fn describe_potion_contains_expected_fields() {
        let s = potion().describe();
        assert!(s.contains("Health Potion"));
        assert!(s.contains("Common"));
        assert!(s.contains("Potion"));
        assert!(s.contains("25"));
    }
}
