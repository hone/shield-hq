use crate::graphql::{Ctx, SHQScalarValue};
use card_side::{CardSide, CardSideInput};
use juniper::{GraphQLEnum, GraphQLInputObject, GraphQLObject};
use serde::Deserialize;

mod basic_power;
mod card_product;
mod card_set;
mod cost;
mod hit_points;
mod keyword;
mod r#trait;

pub mod card_side;
pub use basic_power::BasicPower;
pub use card_product::{
    CardProduct, CardProductInput, CardProductInputBuilder, CardProductInputBuilderError,
};
pub use card_set::{CardSet, CardSetInput, CardSetInputBuilder, CardSetInputBuilderError};
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

#[derive(Clone, Deserialize, GraphQLObject)]
#[graphql(Context = Ctx, scalar = SHQScalarValue)]
#[serde(deny_unknown_fields)]
pub struct Card {
    #[serde(rename = "product")]
    pub products: Vec<CardProduct>,
    #[serde(rename = "side")]
    pub sides: Vec<CardSide>,
    pub aspect: Option<Aspect>,
}

#[derive(GraphQLInputObject)]
#[graphql(scalar = SHQScalarValue)]
pub struct CardInput {
    pub aspect: Option<Option<Aspect>>,
    pub products: Option<Vec<CardProductInput>>,
    pub sides: Option<Vec<CardSideInput>>,
}

#[derive(Clone, Deserialize, GraphQLEnum, PartialEq)]
pub enum Aspect {
    Basic,
    Aggression,
    Leadership,
    Protection,
    Justice,
}

#[derive(Deserialize, GraphQLEnum)]
pub enum CardType {
    Ally,
    Attachment,
    Event,
    Hero,
    Minion,
    Treachery,
    Upgrade,
}

#[derive(Clone, Deserialize, GraphQLEnum)]
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

#[derive(Clone, Deserialize, GraphQLEnum)]
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
