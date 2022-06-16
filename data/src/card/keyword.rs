use lazy_static::lazy_static;
use regex::Regex;
use serde::{
    de::{self, Deserializer, Unexpected, Visitor},
    Deserialize,
};
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Keyword {
    Incite(u8),
    Hinder(u8),
    Quickstrike,
    Stalwart,
    Steady,
    Toughness,
}

impl<'de> Deserialize<'de> for Keyword {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct KeywordVisitor;

        impl<'de> Visitor<'de> for KeywordVisitor {
            type Value = Keyword;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a keyword")
            }

            fn visit_str<E>(self, input: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                lazy_static! {
                    static ref INCITE_RE: Regex = Regex::new(r"Incite ([\d]+)").unwrap();
                    static ref HINDER_RE: Regex = Regex::new(r"Hinder ([\d]+)").unwrap();
                }

                if let Some(caps) = INCITE_RE.captures(input) {
                    if let Some(cap_value) = caps.get(1) {
                        let value = cap_value.as_str().parse::<u8>().map_err(|_| {
                            de::Error::invalid_value(
                                Unexpected::Str(input),
                                &"'Incite X', where X is a number.",
                            )
                        })?;

                        Ok(Keyword::Incite(value))
                    } else {
                        // should not get here if it captures the regex
                        Err(de::Error::invalid_value(
                            Unexpected::Str(input),
                            &"Incite X",
                        ))
                    }
                } else if let Some(caps) = HINDER_RE.captures(input) {
                    if let Some(cap_value) = caps.get(1) {
                        let value = cap_value.as_str().parse::<u8>().map_err(|_| {
                            de::Error::invalid_value(
                                Unexpected::Str(input),
                                &"'Hinder X', where X is a number.",
                            )
                        })?;

                        Ok(Keyword::Hinder(value))
                    } else {
                        // should not get here if it captures the regex
                        Err(de::Error::invalid_value(
                            Unexpected::Str(input),
                            &"Hinder X",
                        ))
                    }
                } else if input == "Quickstrike" {
                    Ok(Keyword::Quickstrike)
                } else if input == "Stalwart" {
                    Ok(Keyword::Stalwart)
                } else if input == "Steady" {
                    Ok(Keyword::Steady)
                } else if input == "Toughness" {
                    Ok(Keyword::Toughness)
                } else {
                    Err(de::Error::invalid_value(
                        Unexpected::Str(input),
                        &"Not a valid Keyword",
                    ))
                }
            }
        }

        deserializer.deserialize_str(KeywordVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Deserialize)]
    struct KeywordDocument {
        pub keywords: Vec<Keyword>,
    }

    fn toml_keyword(keyword: &str) -> Keyword {
        let result: Result<KeywordDocument, _> =
            toml::from_str(&format!(r#"keywords = ["{keyword}"]"#));
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
}
