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
                if let Some(amount) = deserialize_number("Incite", input)? {
                    Ok(Keyword::Incite(amount))
                } else if let Some(amount) = deserialize_number("Hinder", input)? {
                    Ok(Keyword::Hinder(amount))
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

/// Extract the number from a keyword. For instance the X in "Hinder X".
fn deserialize_number<E>(keyword: &str, input: &str) -> Result<Option<u8>, E>
where
    E: de::Error,
{
    let regex = Regex::new(&format!(r"{} ([\d]+)", keyword)).unwrap();

    if let Some(caps) = regex.captures(input) {
        if let Some(cap_value) = caps.get(1) {
            let value = cap_value.as_str().parse::<u8>().map_err(|_| {
                de::Error::invalid_value(
                    Unexpected::Str(input),
                    &format!("'{keyword} X', where X is a number.").as_str(),
                )
            })?;

            Ok(Some(value))
        } else {
            // should not get here if it captures the regex
            Err(de::Error::invalid_value(
                Unexpected::Str(input),
                &format!("{keyword} X").as_str(),
            ))
        }
    } else {
        Ok(None)
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
