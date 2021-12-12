use serde::{Deserialize, Deserializer, Serializer};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Nullable<T> {
    Present(T),
    #[serde(serialize_with = "serialize_null")]
    Null,
}

pub fn serialize_null<S: Serializer>(serializer: S) -> Result<S::Ok, S::Error> {
    serializer.serialize_none()
}

pub fn deserialize_optional_nullable<'de, T, D>(
    deserializer: D,
) -> Result<Option<Nullable<T>>, D::Error>
    where
        D: Deserializer<'de>,
        T: Deserialize<'de>,
{
    Ok(Some(Nullable::deserialize(deserializer)?))
}

impl<T> Nullable<T> {
    pub fn unwrap(self) -> T {
        match self {
            Nullable::Present(data) => data,
            _ => panic!("Nullable is null!"),
        }
    }

    pub fn is_null(&self) -> bool {
        match &self {
            Nullable::Present(_) => false,
            Nullable::Null => true,
        }
    }

    pub fn map<U, F: FnOnce(T) -> U>(self, map: F) -> Nullable<U> {
        match self {
            Nullable::Present(d) => Nullable::Present(map(d)),
            Nullable::Null => Nullable::Null,
        }
    }
}

impl<T> Default for Nullable<T> {
    fn default() -> Self {
        Self::Null
    }
}

impl<T> Into<Option<T>> for Nullable<T> {
    fn into(self) -> Option<T> {
        match self {
            Nullable::Present(data) => Some(data),
            Nullable::Null => None,
        }
    }
}

impl<T> From<Option<T>> for Nullable<T> {
    fn from(data: Option<T>) -> Self {
        match data {
            None => Self::Null,
            Some(data) => Self::Present(data),
        }
    }
}