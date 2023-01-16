use crate::graphql::{filter, SHQScalarValue};
use chrono::NaiveDate;
use juniper::{graphql_object, GraphQLEnum, GraphQLObject};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Document {
    #[serde(rename = "product")]
    pub products: Vec<Product>,
}

#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Product {
    pub name: String,
    pub release_date: NaiveDate,
    pub r#type: ProductType,
    pub code: String,
    pub wave: u32,
    #[serde(default, rename = "set")]
    pub sets: Vec<Set>,
}

#[graphql_object(scalar = SHQScalarValue)]
impl Product {
    fn name(&self) -> &str {
        &self.name
    }

    fn release_date(&self) -> &NaiveDate {
        &self.release_date
    }

    fn r#type(&self) -> &ProductType {
        &self.r#type
    }

    fn code(&self) -> &str {
        &self.code
    }

    fn wave(&self) -> u32 {
        self.wave
    }

    fn sets(&self, name: Option<String>, r#type: Option<SetType>) -> Vec<&Set> {
        let sets: Vec<&Set> = self.sets.iter().collect();

        sets.into_iter()
            .filter(|set| {
                let mut filter = true;

                filter!(filter,
                    &set.name => name,
                    &set.r#type => r#type
                );

                filter
            })
            .collect::<Vec<&Set>>()
    }
}

#[derive(Clone, Deserialize, GraphQLEnum, PartialEq)]
pub enum ProductType {
    #[serde(rename = "Core Set")]
    CoreSet,
    #[serde(rename = "Campaign Expansion")]
    CampaignExpansion,
    #[serde(rename = "Hero Pack")]
    HeroPack,
    #[serde(rename = "Scenario Pack")]
    ScenarioPack,
    Custom,
}

#[derive(Clone, Deserialize, GraphQLObject)]
pub struct Set {
    pub name: String,
    pub r#type: SetType,
}

#[derive(Clone, Deserialize, GraphQLEnum, PartialEq)]
pub enum SetType {
    #[serde(rename = "Hero Signature")]
    HeroSignature,
    #[serde(rename = "Modular Encounter")]
    ModularEncounter,
    Nemesis,
    Villain,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_products() {
        let document: Result<Document, _> = toml::from_str(include_str!("../data/products.toml"));

        assert!(document.is_ok());
    }
}
