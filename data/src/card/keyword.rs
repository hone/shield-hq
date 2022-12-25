use juniper::{graphql_scalar, ParseScalarResult, ParseScalarValue, Value};
use lazy_static::lazy_static;
use regex::Regex;
use serde::{
    de::{self, Deserializer, IntoDeserializer, Unexpected},
    Deserialize,
};
use std::{fmt, str::FromStr};
use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum ParseKeywordError {
    #[error("Not a valid keyword: {0}")]
    UnknownVariant(String),
    #[error("This keyword takes X, which is a natural number: {0}")]
    InvalidValueX(String),
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(remote = "Keyword")]
pub enum Keyword {
    Incite(u8),
    Hinder(u8),
    Quickstrike,
    Stalwart,
    Steady,
    Toughness,
}

impl fmt::Display for Keyword {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = match self {
            Keyword::Incite(n) => format!("Incite {n}"),
            Keyword::Hinder(n) => format!("Hinder {n}"),
            Keyword::Quickstrike => String::from("Quickstrike"),
            Keyword::Stalwart => String::from("Stalwart"),
            Keyword::Steady => String::from("Steady"),
            Keyword::Toughness => String::from("Toughness"),
        };
        write!(f, "{str}")
    }
}

impl FromStr for Keyword {
    type Err = ParseKeywordError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref KEYWORD_RE: Regex = Regex::new(r"([\w]+) ([\d]+)").unwrap();
        }
        if let Some(caps) = KEYWORD_RE.captures(value) {
            // b/c the regex has been captured, these 2 values should be there
            let keyword = caps.get(1).unwrap();
            let x_value = caps
                .get(2)
                .unwrap()
                .as_str()
                .parse::<u8>()
                .map_err(|_| ParseKeywordError::InvalidValueX(String::from(value)))?;

            match keyword.as_str() {
                "Hinder" => Ok(Keyword::Hinder(x_value)),
                "Incite" => Ok(Keyword::Incite(x_value)),
                _ => Err(ParseKeywordError::UnknownVariant(String::from(value))),
            }
        } else {
            match value {
                "Quickstrike" => Ok(Keyword::Quickstrike),
                "Stalwart" => Ok(Keyword::Stalwart),
                "Steady" => Ok(Keyword::Steady),
                "Toughness" => Ok(Keyword::Toughness),
                _ => Err(ParseKeywordError::UnknownVariant(String::from(value))),
            }
        }
    }
}

impl<'de> Deserialize<'de> for Keyword {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        lazy_static! {
            static ref KEYWORD_RE: Regex = Regex::new(r"([\w]+) ([\d]+)").unwrap();
        }
        let s = String::deserialize(deserializer)?;

        if let Some(caps) = KEYWORD_RE.captures(&s) {
            let keyword = caps.get(1).ok_or_else(|| {
                de::Error::invalid_type(
                    Unexpected::Str(&s),
                    &"Keyword expects a word to precede an integer, i.e. Incite 1",
                )
            })?;
            let cap_value = caps.get(2).ok_or_else(|| {
                de::Error::invalid_value(
                    Unexpected::Str(&s),
                    &"Keyword expects an integer to follow, i.e. Incite 1",
                )
            })?;
            let value = cap_value.as_str().parse::<u8>().map_err(|_| {
                de::Error::invalid_value(Unexpected::Str(&s), &"'X', where X is a number.")
            })?;

            match keyword.as_str() {
                "Hinder" => Ok(Keyword::Hinder(value)),
                "Incite" => Ok(Keyword::Incite(value)),
                _ => Err(de::Error::unknown_variant(&s, &["Hinder", "Incite"])),
            }
        } else {
            Keyword::deserialize(s.into_deserializer())
        }
    }
}

#[graphql_scalar]
impl<S> GraphQLScalar for Keyword
where
    S: ScalarValue,
{
    fn resolve(&self) -> Value {
        Value::scalar(self.to_string())
    }

    fn from_input_value(value: InputValue) -> Option<Keyword> {
        if let Some(s) = value.as_string_value() {
            <Keyword as FromStr>::from_str(&s).ok()
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
    use std::str::FromStr;

    #[derive(Debug, Deserialize)]
    struct Document {
        pub keywords: Vec<Keyword>,
    }

    fn toml_keyword(keyword: &str) -> Keyword {
        let result: Result<Document, _> = toml::from_str(&format!(r#"keywords = ["{keyword}"]"#));
        assert!(result.is_ok());

        let mut document = result.unwrap();
        document.keywords.pop().unwrap()
    }

    #[test]
    fn it_parses_incite() {
        assert_eq!(Keyword::Incite(1), toml_keyword("Incite 1"));
    }

    #[test]
    fn it_parses_hinder() {
        assert_eq!(Keyword::Hinder(2), toml_keyword("Hinder 2"));
    }

    #[test]
    fn it_parses_quickstrike() {
        assert_eq!(Keyword::Quickstrike, toml_keyword("Quickstrike"));
    }

    #[test]
    fn it_parses_stalwart() {
        assert_eq!(Keyword::Stalwart, toml_keyword("Stalwart"));
    }

    #[test]
    fn it_parses_steady() {
        assert_eq!(Keyword::Steady, toml_keyword("Steady"));
    }

    #[test]
    fn it_parses_toughness() {
        assert_eq!(Keyword::Toughness, toml_keyword("Toughness"));
    }

    #[test]
    fn it_parses_quickstrike_from_str() {
        assert_eq!(Ok(Keyword::Quickstrike), Keyword::from_str("Quickstrike"))
    }

    #[test]
    fn it_parses_incite_from_str() {
        assert_eq!(Ok(Keyword::Incite(1)), Keyword::from_str("Incite 1"))
    }
}
