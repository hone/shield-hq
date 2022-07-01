use card_side::CardSide;
use serde::Deserialize;

mod cost;
mod keyword;
mod r#trait;

pub mod card_side;
pub use cost::Cost;
pub use keyword::Keyword;
pub use r#trait::Trait;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Document {
    pub card: Vec<Card>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Card {
    pub product: Vec<CardProduct>,
    pub side: Vec<CardSide>,
    pub aspect: Option<Aspect>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CardProduct {
    pub code: String,
    pub positions: Vec<u32>,
    pub set: Option<Vec<CardSet>>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CardSet {
    pub name: String,
    pub positions: Option<Vec<u32>>,
}

#[derive(Deserialize)]
pub enum Aspect {
    Basic,
    Aggression,
    Leadership,
    Protection,
    Justice,
}

#[derive(Deserialize)]
pub enum CardType {
    Ally,
    Attachment,
    Event,
    Hero,
    Minion,
    Treachery,
    Upgrade,
}

#[derive(Deserialize)]
pub enum Resource {
    #[serde(rename = ":energy:")]
    Energy,
    #[serde(rename = ":mental:")]
    Mental,
    #[serde(rename = ":physical:")]
    Physical,
    #[serde(rename = ":wild:")]
    Wild,
}

#[derive(Deserialize)]
pub enum SideSchemeIcon {
    #[serde(rename = ":acceleration:")]
    Acceleration,
    #[serde(rename = ":crisis:")]
    Crisis,
    #[serde(rename = ":hazard:")]
    Hazard,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_cards() {
        let document: Result<Document, _> = toml::from_str(include_str!("../data/core-set.toml"));

        document.unwrap();
    }
}
