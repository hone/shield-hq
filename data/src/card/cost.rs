use serde::{
    de::{self, Deserializer},
    Deserialize,
};

#[derive(Debug, PartialEq)]
pub enum Cost {
    Number(u8),
    X,
}

impl<'de> Deserialize<'de> for Cost {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        if let Ok(number) = s.parse::<u8>() {
            Ok(Cost::Number(number))
        } else if s == "X" {
            Ok(Cost::X)
        } else {
            Err(de::Error::unknown_variant(&s, &["Number", "X"]))
        }
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
        let result: Result<Document, toml::de::Error> = toml::from_str(r#"cost = "3""#);
        let doc = result.unwrap();

        assert_eq!(Cost::Number(3), doc.cost);
    }

    #[test]
    fn it_parses_x() {
        let result: Result<Document, toml::de::Error> = toml::from_str(r#"cost = "X""#);
        let doc = result.unwrap();

        assert_eq!(Cost::X, doc.cost);
    }
}
