use juniper::{graphql_scalar, ParseScalarResult, ParseScalarValue, Value};
use lazy_static::lazy_static;
use regex::Regex;
use serde::{
    de::{self, Deserializer, Unexpected, Visitor},
    Deserialize,
};
use std::{fmt, str::FromStr};
use thiserror::Error;

struct HitPointsVisitor;

impl<'de> Visitor<'de> for HitPointsVisitor {
    type Value = HitPoints;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Integer or Integer:player:")
    }

    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(HitPoints::Number(value as u8))
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        <HitPoints as FromStr>::from_str(value).map_err(|_| {
            de::Error::invalid_value(
                Unexpected::Str(value),
                &"takes an Integer or Integer:player:",
            )
        })
    }
}

#[derive(Debug, Error, PartialEq)]
#[error("{0} is not a Number or per player")]
pub struct ParseHitPointsError(String);

#[derive(Clone, Debug, PartialEq)]
pub enum HitPoints {
    Number(u8),
    PerPlayer(u8),
}

impl fmt::Display for HitPoints {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = match self {
            HitPoints::Number(n) => n.to_string(),
            HitPoints::PerPlayer(n) => format!("{n} per Player"),
        };
        write!(f, "{str}")
    }
}

impl FromStr for HitPoints {
    type Err = ParseHitPointsError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref HIT_POINTS_RE: Regex = Regex::new(r"([\d]+):player:").unwrap();
        }

        if let Ok(num) = value.parse::<u8>() {
            Ok(HitPoints::Number(num))
        } else if let Some(caps) = HIT_POINTS_RE.captures(&value) {
            // b/c of the regex, this should always unwrap()
            let cap_value = caps.get(1).unwrap();
            let number = cap_value.as_str().parse::<u8>().unwrap();
            Ok(HitPoints::PerPlayer(number))
        } else {
            Err(ParseHitPointsError(String::from(value)))
        }
    }
}

impl<'de> Deserialize<'de> for HitPoints {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(HitPointsVisitor)
    }
}

#[graphql_scalar]
impl<S> GraphQLScalar for HitPoints
where
    S: ScalarValue,
{
    fn resolve(&self) -> Value {
        Value::scalar(self.to_string())
    }

    fn from_input_value(value: InputValue) -> Option<HitPoints> {
        if let Some(s) = value.as_string_value() {
            <HitPoints as FromStr>::from_str(&s).ok()
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
        pub hit_points: HitPoints,
    }

    fn toml_hit_points(hit_points: &str) -> HitPoints {
        let result: Result<Document, _> = toml::from_str(&format!("hit_points = {hit_points}"));
        assert!(result.is_ok());

        let document = result.unwrap();
        document.hit_points
    }

    #[test]
    fn it_parses_as_integer() {
        assert_eq!(HitPoints::Number(12), toml_hit_points("12"))
    }

    #[test]
    fn it_parses_number_from_string() {
        assert_eq!(HitPoints::Number(12), toml_hit_points(r#""12""#))
    }

    #[test]
    fn it_parses_per_player() {
        assert_eq!(HitPoints::PerPlayer(4), toml_hit_points(r#""4:player:""#))
    }
}
