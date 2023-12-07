use std::fmt::{self, Debug, Display};

use serde::{de::DeserializeOwned, Deserializer};

pub struct Secret<T>(T);

impl<T> Debug for Secret<T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Secret(...)")
    }
}

impl<T> AsRef<T> for Secret<T> {
    fn as_ref(&self) -> &T {
        &self.0
    }
}

impl<T> Clone for Secret<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Secret(self.0.clone())
    }
}

impl<T> Display for Secret<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <Self as Debug>::fmt(&self, f)
    }
}

impl<T> PartialEq for Secret<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<'de, T> serde::Deserialize<'de> for Secret<T>
where
    T: DeserializeOwned + 'de,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        T::deserialize(deserializer).map(Secret)
    }
}

impl<'de, T> serde::Serialize for Secret<T>
where
    T: serde::Serialize + 'de,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        T::serialize(&self.0, serializer)
    }
}
