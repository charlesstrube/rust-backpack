#[derive(Debug)]
pub enum ItemWeightError {
    WeightIsZero,
}

#[derive(Debug, PartialEq)]
pub struct ItemWeight(u32);

impl ItemWeight {
    pub fn value(&self) -> u32 {
        self.0
    }
}

impl TryFrom<u32> for ItemWeight {
    type Error = ItemWeightError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if value <= 0 {
            return Err(ItemWeightError::WeightIsZero);
        }
        Ok(Self(value))
    }
}
