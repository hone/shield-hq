use juniper::{graphql_scalar, ParseScalarResult, ParseScalarValue, Value};
use serde::{
    self,
    de::{self, Deserializer, Unexpected, Visitor},
    Deserialize,
};
use std::{fmt, str::FromStr};
use thiserror::Error;

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
        <BasicPower as FromStr>::from_str(value)
            .map_err(|_| de::Error::invalid_value(Unexpected::Str(value), &"is not a Number or X"))
    }
}

#[derive(Debug, Error, PartialEq)]
#[error("{0} is not a Number or X")]
pub struct ParseBasicPowerError(String);

#[derive(Debug, PartialEq)]
pub enum BasicPower {
    Number(u8),
    X,
}

impl fmt::Display for BasicPower {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = match self {
            BasicPower::Number(n) => n.to_string(),
            BasicPower::X => String::from("X"),
        };
        write!(f, "{str}")
    }
}

impl FromStr for BasicPower {
    type Err = ParseBasicPowerError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if let Ok(num) = value.parse::<u8>() {
            Ok(BasicPower::Number(num))
        } else if value == "X" {
            Ok(BasicPower::X)
        } else {
            Err(ParseBasicPowerError(String::from(value)))
        }
    }
}

impl<'de> Deserialize<'de> for BasicPower {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(BasicPowerVisitor)
    }
}

#[graphql_scalar]
impl<S> GraphQLScalar for BasicPower
where
    S: ScalarValue,
{
    fn resolve(&self) -> Value {
        Value::scalar(self.to_string())
    }

    fn from_input_value(value: InputValue) -> Option<BasicPower> {
        if let Some(s) = value.as_string_value() {
            <BasicPower as FromStr>::from_str(&s).ok()
        } else {
            None
        }
    }

    fn from_str<'a>(value: ScalarToken<'a>) -> ParseScalarResult<'a, S> {
        <String as ParseScalarValue<S>>::from_str(value)
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

    #[test]
    fn it_parses_number_from_str() {
        assert_eq!(
            Ok(BasicPower::Number(1)),
            <BasicPower as FromStr>::from_str("1")
        );
    }

    #[test]
    fn it_parses_x_from_str() {
        assert_eq!(Ok(BasicPower::X), <BasicPower as FromStr>::from_str("X"));
    }
}
