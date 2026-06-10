#[derive(Debug, Clone, PartialEq)]
pub struct ItemName(String);

#[derive(Debug)]
pub enum ItemNameError {
    NameIsEmpty,
}

impl ItemName {
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl TryFrom<&str> for ItemName {
    type Error = ItemNameError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_string())
    }
}

impl TryFrom<String> for ItemName {
    type Error = ItemNameError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(ItemNameError::NameIsEmpty);
        }
        Ok(Self(value))
    }
}
