use lazy_static::lazy_static;
use regex::Regex;
use serde::{
    de::{self, Deserializer, IntoDeserializer, Unexpected},
    Deserialize,
};

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
                de::Error::invalid_value(
                    Unexpected::Str(&s),
                    &format!("'X', where X is a number.").as_str(),
                )
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
