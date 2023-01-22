use crate::graphql::{filter, SHQScalarValue};
use derive_builder::Builder;
use juniper::{GraphQLInputObject, GraphQLObject};
use serde::Deserialize;
use std::collections::HashSet;

#[derive(Builder, Clone, GraphQLInputObject)]
#[graphql(scalar = SHQScalarValue)]
pub struct CardSetInput {
    #[builder(default)]
    pub name: Option<String>,
    #[builder(default)]
    pub positions: Option<Option<Vec<u32>>>,
}

#[derive(Clone, Deserialize, GraphQLObject)]
#[graphql(scalar = SHQScalarValue)]
#[serde(deny_unknown_fields)]
pub struct CardSet {
    pub name: String,
    pub positions: Option<Vec<u32>>,
}

impl CardSet {
    pub fn included(&self, input: &CardSetInput) -> bool {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn included_name() {
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
    fn not_included_name() {
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
    fn included_positions() {
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
    fn not_included_positions() {
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
    fn included_none() {
        let sinister_set = CardSet {
            name: String::from("Soemthing Sinister"),
            positions: Some(vec![1, 2]),
        };
        let input = CardSetInputBuilder::default().build().unwrap();

        assert_eq!(true, sinister_set.included(&input));
    }
}
