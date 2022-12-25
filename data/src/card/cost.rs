use juniper::{graphql_scalar, ParseScalarResult, ParseScalarValue, Value};
use serde::{
    de::{self, Deserializer, Unexpected, Visitor},
    Deserialize,
};
use std::{fmt, str::FromStr};
use thiserror::Error;

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
        <Cost as FromStr>::from_str(value)
            .map_err(|_| de::Error::invalid_value(Unexpected::Str(value), &"is not a Number or X"))
    }
}

#[derive(Debug, Error)]
#[error("{0} is not a Number or X")]
pub struct ParseCostError(String);

#[derive(Debug, PartialEq)]
pub enum Cost {
    Number(u8),
    X,
}

impl fmt::Display for Cost {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = match self {
            Cost::Number(n) => n.to_string(),
            Cost::X => String::from("X"),
        };
        write!(f, "{str}")
    }
}

impl FromStr for Cost {
    type Err = ParseCostError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if let Ok(num) = value.parse::<u8>() {
            Ok(Cost::Number(num))
        } else if value == "X" {
            Ok(Cost::X)
        } else {
            Err(ParseCostError(String::from(value)))
        }
    }
}

impl<'de> Deserialize<'de> for Cost {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(CostVisitor)
    }
}

#[graphql_scalar]
impl<S> GraphQLScalar for Cost
where
    S: ScalarValue,
{
    fn resolve(&self) -> Value {
        Value::scalar(self.to_string())
    }

    fn from_input_value(value: InputValue) -> Option<Cost> {
        if let Some(s) = value.as_string_value() {
            <Cost as FromStr>::from_str(&s).ok()
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
