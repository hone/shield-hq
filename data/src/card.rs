use crate::graphql::{filter, SHQScalarValue};
use card_side::{CardSide, CardSideInput};
use derive_builder::Builder;
use juniper::{graphql_object, GraphQLEnum, GraphQLInputObject, GraphQLObject};
use serde::Deserialize;
use std::collections::HashSet;

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

#[derive(Clone, Deserialize, GraphQLObject)]
#[graphql(scalar = SHQScalarValue)]
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

#[derive(Builder, Clone, GraphQLInputObject)]
#[graphql(scalar = SHQScalarValue)]
pub struct CardProductInput {
    #[builder(default)]
    pub code: Option<String>,
    #[builder(default)]
    pub positions: Option<Vec<u32>>,
    #[builder(default)]
    pub sets: Option<Option<Vec<CardSetInput>>>,
}

#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CardProduct {
    pub code: String,
    pub positions: Vec<u32>,
    #[serde(rename = "set")]
    pub sets: Option<Vec<CardSet>>,
}

#[graphql_object(scalar = SHQScalarValue)]
impl CardProduct {
    fn code(&self) -> &str {
        &self.code
    }

    fn positions(&self) -> &Vec<u32> {
        &self.positions
    }

    fn sets(&self, name: Option<String>) -> Option<Vec<&CardSet>> {
        if let Some(sets) = &self.sets {
            Some(
                sets.into_iter()
                    .filter(|set| {
                        let mut filter = true;

                        filter!(filter, &set.name => name);

                        filter
                    })
                    .collect(),
            )
        } else {
            None
        }
    }
}

impl CardProduct {
    pub fn included(&self, input: &CardProductInput) -> bool {
        let mut filter = true;

        filter!(filter, &self.code => input.code);
        if let Some(positions) = &input.positions {
            let a: HashSet<&u32> = positions.iter().collect();
            let b: HashSet<&u32> = self.positions.iter().collect();

            filter = filter && a.intersection(&b).next().is_some();
        }
        if let Some(input_sets) = &input.sets {
            if let Some(input_sets) = input_sets {
                if let Some(self_sets) = &self.sets {
                    filter = filter
                        && !input_sets
                            .iter()
                            .filter(|input_set| self_sets.iter().any(|set| set.included(input_set)))
                            .collect::<Vec<_>>()
                            .is_empty();
                } else {
                    filter = false;
                }
            }
        }

        filter
    }
}

#[derive(Clone, Deserialize, GraphQLObject)]
#[graphql(scalar = SHQScalarValue)]
#[serde(deny_unknown_fields)]
pub struct CardSet {
    pub name: String,
    pub positions: Option<Vec<u32>>,
}

impl CardSet {
    fn included(&self, input: &CardSetInput) -> bool {
        let mut filter = true;

        filter!(filter, &self.name => input.name);
        if let Some(input_positions) = &input.positions {
            if input_positions.is_some() && self.positions.is_some() {
                let input_positions = input_positions.as_ref().unwrap();
                let a: HashSet<&u32> = input_positions.iter().collect();
                let b: HashSet<&u32> = self.positions.as_ref().unwrap().iter().collect();

                filter = a.intersection(&b).next().is_some() && filter;
            } else if input_positions.is_none() && self.positions.is_none() {
                filter = true && filter;
            } else {
                filter = false;
            }
        }

        filter
    }
}

#[derive(Builder, Clone, GraphQLInputObject)]
#[graphql(scalar = SHQScalarValue)]
pub struct CardSetInput {
    #[builder(default)]
    pub name: Option<String>,
    #[builder(default)]
    pub positions: Option<Option<Vec<u32>>>,
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

    #[test]
    fn card_product_included_none() {
        let card_set = CardSet {
            name: String::from("Something Sinister"),
            positions: Some(vec![1, 2]),
        };
        let card_product = CardProduct {
            code: String::from("MC01en"),
            positions: vec![83, 84],
            sets: Some(vec![card_set]),
        };
        let input_none = CardProductInputBuilder::default().build().unwrap();

        assert_eq!(true, card_product.included(&input_none));
    }

    #[test]
    fn card_product_included_code() {
        let card_set = CardSet {
            name: String::from("Something Sinister"),
            positions: Some(vec![1, 2]),
        };
        let card_product = CardProduct {
            code: String::from("MC01en"),
            positions: vec![83, 84],
            sets: Some(vec![card_set]),
        };
        let code_input_included = CardProductInputBuilder::default()
            .code(Some(String::from("MC01en")))
            .build()
            .unwrap();
        let code_input_not_included = CardProductInputBuilder::default()
            .code(Some(String::from("MC02en")))
            .build()
            .unwrap();

        assert_eq!(true, card_product.included(&code_input_included));
        assert_eq!(false, card_product.included(&code_input_not_included));
    }

    #[test]
    fn card_product_included_positions() {
        let card_set = CardSet {
            name: String::from("Something Sinister"),
            positions: Some(vec![1, 2]),
        };
        let card_product = CardProduct {
            code: String::from("MC01en"),
            positions: vec![83, 84],
            sets: Some(vec![card_set]),
        };
        let positions_input_included = CardProductInputBuilder::default()
            .positions(Some(vec![83]))
            .build()
            .unwrap();
        let positions_input_not_included = CardProductInputBuilder::default()
            .positions(Some(vec![90]))
            .build()
            .unwrap();
        let positions_input_partial_include = CardProductInputBuilder::default()
            .positions(Some(vec![83, 90]))
            .build()
            .unwrap();

        assert_eq!(true, card_product.included(&positions_input_included));
        assert_eq!(false, card_product.included(&positions_input_not_included));
        assert_eq!(
            true,
            card_product.included(&positions_input_partial_include)
        );
    }

    #[test]
    fn card_product_input_included_sets_name() {
        let sinister_set = CardSet {
            name: String::from("Something Sinister"),
            positions: Some(vec![1, 2]),
        };
        let sit_com_set = CardSet {
            name: String::from("Sit-Com"),
            positions: Some(vec![3, 4]),
        };
        let card_product = CardProduct {
            code: String::from("MC01en"),
            positions: vec![83, 84],
            sets: Some(vec![sinister_set, sit_com_set]),
        };
        let input = CardProductInputBuilder::default()
            .sets(Some(Some(vec![CardSetInputBuilder::default()
                .name(Some(String::from("Something Sinister")))
                .build()
                .unwrap()])))
            .build()
            .unwrap();
        let input_or = CardProductInputBuilder::default()
            .sets(Some(Some(vec![
                CardSetInputBuilder::default()
                    .name(Some(String::from("Something Sinister")))
                    .build()
                    .unwrap(),
                CardSetInputBuilder::default()
                    .name(Some(String::from("None")))
                    .build()
                    .unwrap(),
            ])))
            .build()
            .unwrap();

        assert_eq!(true, card_product.included(&input));
        assert_eq!(true, card_product.included(&input_or));
    }

    #[test]
    fn card_product_input_not_included_sets_name() {
        let sinister_set = CardSet {
            name: String::from("Something Sinister"),
            positions: Some(vec![1, 2]),
        };
        let sit_com_set = CardSet {
            name: String::from("Sit-Com"),
            positions: Some(vec![3, 4]),
        };
        let card_product = CardProduct {
            code: String::from("MC01en"),
            positions: vec![83, 84],
            sets: Some(vec![sinister_set, sit_com_set]),
        };
        let input = CardProductInputBuilder::default()
            .sets(Some(Some(vec![CardSetInputBuilder::default()
                .name(Some(String::from("Not Included")))
                .build()
                .unwrap()])))
            .build()
            .unwrap();

        assert_eq!(false, card_product.included(&input));
    }

    #[test]
    fn card_set_input_included_name() {
        let sinister_set = CardSet {
            name: String::from("Something Sinister"),
            positions: Some(vec![1, 2]),
        };
        let input = CardSetInputBuilder::default()
            .name(Some(String::from("Something Sinister")))
            .build()
            .unwrap();

        assert_eq!(true, sinister_set.included(&input));
    }

    #[test]
    fn card_set_input_not_included_name() {
        let sinister_set = CardSet {
            name: String::from("Something Sinister"),
            positions: Some(vec![1, 2]),
        };
        let input = CardSetInputBuilder::default()
            .name(Some(String::from("Not Included")))
            .build()
            .unwrap();

        assert_eq!(false, sinister_set.included(&input));
    }

    #[test]
    fn card_set_input_included_positions() {
        let sinister_set = CardSet {
            name: String::from("Something Sinister"),
            positions: Some(vec![1, 2]),
        };
        let none_set = CardSet {
            name: String::from("None Set"),
            positions: None,
        };
        let input = CardSetInputBuilder::default()
            .positions(Some(Some(vec![1])))
            .build()
            .unwrap();
        let input2 = CardSetInputBuilder::default()
            .positions(Some(Some(vec![1, 4])))
            .build()
            .unwrap();
        let input3 = CardSetInputBuilder::default()
            .positions(Some(None))
            .build()
            .unwrap();

        assert_eq!(true, sinister_set.included(&input));
        assert_eq!(true, sinister_set.included(&input2));
        assert_eq!(true, none_set.included(&input3));
    }

    #[test]
    fn card_set_input_not_included_positions() {
        let sinister_set = CardSet {
            name: String::from("Something Sinister"),
            positions: Some(vec![1, 2]),
        };
        let none_set = CardSet {
            name: String::from("None Set"),
            positions: None,
        };
        let input = CardSetInputBuilder::default()
            .positions(Some(Some(vec![3])))
            .build()
            .unwrap();
        let input_none = CardSetInputBuilder::default()
            .positions(Some(None))
            .build()
            .unwrap();

        assert_eq!(false, sinister_set.included(&input));
        assert_eq!(false, sinister_set.included(&input_none));
        assert_eq!(false, none_set.included(&input));
    }

    #[test]
    fn card_set_input_none() {
        let sinister_set = CardSet {
            name: String::from("Soemthing Sinister"),
            positions: Some(vec![1, 2]),
        };
        let input = CardSetInputBuilder::default().build().unwrap();

        assert_eq!(true, sinister_set.included(&input));
    }
}
