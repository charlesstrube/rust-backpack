use std::ops::Deref;

#[derive(Debug, Clone, PartialEq)]
pub struct ItemName(String);

impl ItemName {
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl From<&str> for ItemName {
    fn from(value: &str) -> Self {
        Self(value.into())
    }
}

impl From<String> for ItemName {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl Deref for ItemName {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        self.0.as_str()
    }
}

impl AsRef<str> for ItemName {
    fn as_ref(&self) -> &str {
        &self.0.as_str()
    }
}
