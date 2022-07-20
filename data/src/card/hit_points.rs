use lazy_static::lazy_static;
use regex::Regex;
use serde::{
    de::{self, Deserializer, Unexpected, Visitor},
    Deserialize,
};
use std::fmt;

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
        lazy_static! {
            static ref HIT_POINTS_RE: Regex = Regex::new(r"([\d]+):player:").unwrap();
        }

        if let Ok(num) = value.parse::<u8>() {
            Ok(HitPoints::Number(num))
        } else if let Some(caps) = HIT_POINTS_RE.captures(&value) {
            let cap_value = caps.get(1).ok_or_else(|| {
                de::Error::invalid_value(
                    Unexpected::Str(value),
                    &"takes an Integer or Integer:player:",
                )
            })?;
            let number = cap_value.as_str().parse::<u8>().map_err(|_| {
                de::Error::invalid_value(Unexpected::Str(value), &"X, where X is a number.")
            })?;

            Ok(HitPoints::PerPlayer(number))
        } else {
            Err(de::Error::invalid_value(
                Unexpected::Str(value),
                &"takes an Integer or Integer:player:",
            ))
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum HitPoints {
    Number(u8),
    PerPlayer(u8),
}

impl<'de> Deserialize<'de> for HitPoints {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(HitPointsVisitor)
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
