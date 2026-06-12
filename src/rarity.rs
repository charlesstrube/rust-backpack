use std::fmt::Debug;

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum Rarity {
    Common,
    Rare,
    Epic,
    Legendary,
}

// impl Debug for Rarity {
//     fn fmt(&self, f: &mut Formatter) -> Result {
//         match self {
//             Rarity::Common => write!(f, "Common"),
//             Rarity::Rare => write!(f, "Rare"),
//             Rarity::Epic => write!(f, "Epic"),
//             Rarity::Legendary => write!(f, "Legendary"),
//         }
//     }
// }

// impl PartialEq for Rarity {
//     fn eq(&self, other: &Self) -> bool {
//         matches!(
//             (self, other),
//             (Rarity::Common, Rarity::Common)
//                 | (Rarity::Rare, Rarity::Rare)
//                 | (Rarity::Epic, Rarity::Epic)
//                 | (Rarity::Legendary, Rarity::Legendary)
//         )
//     }
// }

// impl Copy for Rarity {}

// impl Clone for Rarity {
//     fn clone(&self) -> Self {
//         *self
//     }
// }
