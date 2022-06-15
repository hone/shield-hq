use lazy_static::lazy_static;
use regex::Regex;
use serde::{de, Deserialize};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Keyword {
    pub incite: Option<u8>,
    pub hinder: Option<u8>,
    #[serde(default)]
    pub quickstrike: bool,
    #[serde(default)]
    pub stalwart: bool,
    #[serde(default)]
    pub steady: bool,
    #[serde(default)]
    pub toughness: bool,
}

#[derive(Debug, PartialEq)]
pub enum KeywordEnum {
    Incite(u8),
    Hinder(u8),
    Quickstrike,
    Stalwart,
    Steady,
    Toughness,
}

impl TryFrom<&str> for KeywordEnum {
    type Error = &'static str;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        lazy_static! {
            static ref INCITE_RE: Regex = Regex::new(r"Incite ([\d]+)").unwrap();
            static ref HINDER_RE: Regex = Regex::new(r"Hinder ([\d]+)").unwrap();
        }

        if let Some(caps) = INCITE_RE.captures(input) {
            if let Some(cap_value) = caps.get(1) {
                let value = cap_value
                    .as_str()
                    .parse::<u8>()
                    .map_err(|_| "'Incite X', where X is a number.")?;

                Ok(KeywordEnum::Incite(value))
            } else {
                // should not get here if it captures the regex
                Err("Incite X")
            }
        } else if let Some(caps) = HINDER_RE.captures(input) {
            if let Some(cap_value) = caps.get(1) {
                let value = cap_value
                    .as_str()
                    .parse::<u8>()
                    .map_err(|_| "'Hinder X', where X is a number.")?;

                Ok(KeywordEnum::Hinder(value))
            } else {
                // should not get here if it captures the regex
                Err("Hinder X")
            }
        } else if input == "Quickstrike" {
            Ok(KeywordEnum::Quickstrike)
        } else if input == "Stalwart" {
            Ok(KeywordEnum::Stalwart)
        } else if input == "Steady" {
            Ok(KeywordEnum::Steady)
        } else if input == "Toughness" {
            Ok(KeywordEnum::Toughness)
        } else {
            Err("Not a valid Keyword: {value}")
        }
    }
}

fn deserialize_incite<'de, D>(deserialize: D) -> Result<u8, D::Error>
where
    D: de::Deserializer<'de>,
{
    let buf = String::deserialize(deserialize)?;

    lazy_static! {
        static ref INCITE_RE: Regex = Regex::new(r"Incite ([\d]+)").unwrap();
    }

    if let Some(caps) = INCITE_RE.captures(&buf) {
        if let Some(cap_value) = caps.get(1) {
            let value = cap_value.as_str().parse::<u8>().map_err(|_| {
                de::Error::invalid_value(
                    de::Unexpected::Str(&buf),
                    &"'Incite X', where X is a number.",
                )
            })?;

            return Ok(value);
        }
    }

    Err(de::Error::invalid_value(
        de::Unexpected::Str(&buf),
        &"Incite X",
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    //#[test]
    //fn it_parses_incite() {
    //    let result: Result<KeywordEnum, _> = toml::from_str("\"Incite 1\"");

    //    let incite = result.unwrap();
    //    assert_eq!(KeywordEnum::Incite(1), incite);
    //}
}
