use serde::{
    de::{self, Deserializer, Unexpected, Visitor},
    Deserialize,
};
use std::fmt;

struct CostVisitor;

impl<'de> Visitor<'de> for CostVisitor {
    type Value = Cost;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("not a Number or X")
    }

    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Cost::Number(value as u8))
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if let Ok(num) = value.parse::<u8>() {
            Ok(Cost::Number(num))
        } else if value == "X" {
            Ok(Cost::X)
        } else {
            Err(de::Error::invalid_value(
                Unexpected::Str(value),
                &"is not a Number or X",
            ))
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Cost {
    Number(u8),
    X,
}

impl<'de> Deserialize<'de> for Cost {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(CostVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Deserialize)]
    struct Document {
        pub cost: Cost,
    }

    #[test]
    fn it_parses_number() {
        let result: Result<Document, _> = toml::from_str(r#"cost = 3"#);
        let doc = result.unwrap();

        assert_eq!(Cost::Number(3), doc.cost);
    }

    #[test]
    fn it_parses_number_from_string() {
        let result: Result<Document, _> = toml::from_str(r#"cost = "3""#);
        let doc = result.unwrap();

        assert_eq!(Cost::Number(3), doc.cost);
    }

    #[test]
    fn it_parses_x() {
        let result: Result<Document, _> = toml::from_str(r#"cost = "X""#);
        let doc = result.unwrap();

        assert_eq!(Cost::X, doc.cost);
    }
}
