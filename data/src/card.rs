use card_side::CardSide;
use serde::Deserialize;

mod basic_power;
mod cost;
mod hit_points;
mod keyword;
mod r#trait;

pub mod card_side;
pub use basic_power::BasicPower;
pub use cost::Cost;
pub use hit_points::HitPoints;
pub use keyword::Keyword;
pub use r#trait::Trait;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Document {
    #[serde(rename = "card")]
    pub cards: Vec<Card>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Card {
    #[serde(rename = "product")]
    pub products: Vec<CardProduct>,
    #[serde(rename = "side")]
    pub sides: Vec<CardSide>,
    pub aspect: Option<Aspect>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CardProduct {
    pub code: String,
    pub positions: Vec<u32>,
    #[serde(rename = "set")]
    pub sets: Option<Vec<CardSet>>,
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
    use crate::product;

    #[test]
    fn it_parses_cards() {
        let product_document: Result<product::Document, _> =
            toml::from_str(include_str!("../data/products.toml"));

        let document: Result<Document, _> = toml::from_str(include_str!("../data/core-set.toml"));

        let products: Vec<product::Product> = product_document.unwrap().products;
        let cards: Vec<Card> = document.unwrap().cards;

        for card in cards.iter() {
            for card_product in card.products.iter() {
                // check that every card product exists
                if let Some(product) = products
                    .iter()
                    .find(|product| product.code == card_product.code)
                {
                    if let Some(sets) = &card_product.sets {
                        for card_set in sets.iter() {
                            assert!(
                                product.sets.iter().any(|set| set.name == card_set.name),
                                "Could not find product set {}",
                                card_set.name
                            );
                        }
                    }
                } else {
                    panic!("Could not find product {}", card_product.code);
                }
            }
        }
    }
}
