use serde::{
    self,
    de::{self, Deserializer, Unexpected, Visitor},
    Deserialize,
};
use std::fmt;

struct BasicPowerVisitor;

impl<'de> Visitor<'de> for BasicPowerVisitor {
    type Value = BasicPower;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an unsigned integer or X")
    }

    // toml-rs treats Integers as i64
    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(BasicPower::Number(value as u8))
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if let Ok(num) = value.parse::<u8>() {
            Ok(BasicPower::Number(num))
        } else if value == "X" {
            Ok(BasicPower::X)
        } else {
            Err(de::Error::invalid_value(
                Unexpected::Str(value),
                &"is not a Number or X",
            ))
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum BasicPower {
    Number(u8),
    X,
}

impl<'de> Deserialize<'de> for BasicPower {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(BasicPowerVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Deserialize)]
    struct Document {
        pub atk: BasicPower,
    }

    #[test]
    fn it_parses_number() {
        let result: Result<Document, _> = toml::from_str(r#"atk = 1"#);
        let doc = result.unwrap();

        assert_eq!(BasicPower::Number(1), doc.atk);
    }

    #[test]
    fn it_parses_number_from_string() {
        let result: Result<Document, _> = toml::from_str(r#"atk = "1""#);
        let doc = result.unwrap();

        assert_eq!(BasicPower::Number(1), doc.atk);
    }

    #[test]
    fn it_parses_x() {
        let result: Result<Document, _> = toml::from_str(r#"atk = "X""#);
        let doc = result.unwrap();

        assert_eq!(BasicPower::X, doc.atk);
    }
}
