use crate::graphql::{filter, Ctx, SHQScalarValue};
use crate::{
    card::{CardSet, CardSetInput},
    product::{ProductType, Set as ProductSet},
};
use chrono::NaiveDate;
use derive_builder::Builder;
use juniper::{graphql_object, GraphQLInputObject};
use serde::Deserialize;
use std::collections::HashSet;

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

#[graphql_object(Context = Ctx, scalar = SHQScalarValue)]
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

    fn name(&self, context: &Ctx) -> Option<&String> {
        context.product(&self.code).map(|product| &product.name)
    }

    fn release_date(&self, context: &Ctx) -> Option<&NaiveDate> {
        context
            .product(&self.code)
            .map(|product| &product.release_date)
    }

    fn r#type(&self, context: &Ctx) -> Option<&ProductType> {
        context.product(&self.code).map(|product| &product.r#type)
    }

    fn wave(&self, context: &Ctx) -> Option<&u32> {
        context.product(&self.code).map(|product| &product.wave)
    }

    fn product_sets(&self, context: &Ctx) -> Option<&Vec<ProductSet>> {
        context.product(&self.code).map(|product| &product.sets)
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::CardSetInputBuilder;

    #[test]
    fn included_none() {
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
    fn included_code() {
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
    fn included_positions() {
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
    fn input_included_sets_name() {
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
    fn input_not_included_sets_name() {
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
}
