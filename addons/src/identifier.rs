use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct AddonIdentifier {
    pub namespace: String,
    pub name: String,
}

impl Serialize for AddonIdentifier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!("{}:{}", self.namespace, self.name))
    }
}

impl<'de> Deserialize<'de> for AddonIdentifier {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let str = String::deserialize(deserializer)?;

        match str.split_once(":") {
            None => Err(Error::custom(
                "Expected addon identifier to be seperated by a \":\".",
            )),
            Some((namespace, name)) => Ok(Self {
                namespace: namespace.to_string(),
                name: name.to_string(),
            }),
        }
    }
}
