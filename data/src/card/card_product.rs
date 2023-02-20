use crate::graphql::{filter, filter_context, Ctx, SHQScalarValue};
use crate::{
    card::{CardSet, CardSetInput},
    product::ProductType,
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
    // Product Fields
    #[builder(default)]
    pub name: Option<String>,
    #[builder(default)]
    pub release_date: Option<NaiveDate>,
    #[builder(default)]
    pub r#type: Option<ProductType>,
    #[builder(default)]
    pub wave: Option<u32>,
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
        self.name(context)
    }

    fn release_date(&self, context: &Ctx) -> Option<&NaiveDate> {
        self.release_date(context)
    }

    fn r#type(&self, context: &Ctx) -> Option<&ProductType> {
        self.r#type(context)
    }

    fn wave(&self, context: &Ctx) -> Option<&u32> {
        self.wave(context)
    }
}

impl CardProduct {
    pub fn name<'a>(&self, context: &'a Ctx) -> Option<&'a String> {
        context.product(&self.code).map(|product| &product.name)
    }

    pub fn release_date<'a>(&self, context: &'a Ctx) -> Option<&'a NaiveDate> {
        context
            .product(&self.code)
            .map(|product| &product.release_date)
    }

    pub fn r#type<'a>(&self, context: &'a Ctx) -> Option<&'a ProductType> {
        context.product(&self.code).map(|product| &product.r#type)
    }

    pub fn wave<'a>(&self, context: &'a Ctx) -> Option<&'a u32> {
        context.product(&self.code).map(|product| &product.wave)
    }

    pub fn included(&self, input: &CardProductInput, context: &Ctx) -> bool {
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
                            .filter(|input_set| {
                                self_sets.iter().any(|set| set.included(input_set, context))
                            })
                            .collect::<Vec<_>>()
                            .is_empty();
                } else {
                    filter = false;
                }
            }
        }
        filter_context!(filter,
            self.name(context) => input.name,
            self.release_date(context) => input.release_date,
            self.r#type(context) => input.r#type,
            self.wave(context) => input.wave
        );

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

        assert_eq!(true, card_product.included(&input_none, &Ctx::default()));
    }

    #[test]
    fn included_code() {
        let ctx = Ctx::default();
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

        assert_eq!(true, card_product.included(&code_input_included, &ctx));
        assert_eq!(false, card_product.included(&code_input_not_included, &ctx));
    }

    #[test]
    fn included_positions() {
        let ctx = Ctx::default();
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

        assert_eq!(true, card_product.included(&positions_input_included, &ctx));
        assert_eq!(
            false,
            card_product.included(&positions_input_not_included, &ctx)
        );
        assert_eq!(
            true,
            card_product.included(&positions_input_partial_include, &ctx)
        );
    }

    #[test]
    fn input_included_sets_name() {
        let ctx = Ctx::default();
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

        assert_eq!(true, card_product.included(&input, &ctx));
        assert_eq!(true, card_product.included(&input_or, &ctx));
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

        assert_eq!(false, card_product.included(&input, &Ctx::default()));
    }
}
