use serde::Deserialize;

#[derive(Deserialize)]
pub struct Document {
    #[serde(default)]
    pub product: Vec<Product>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Product {
    pub name: String,
    pub release_date: chrono::naive::NaiveDate,
    pub r#type: ProductType,
    pub code: String,
    pub wave: u32,
    #[serde(default)]
    pub set: Vec<Set>,
}

#[derive(Deserialize)]
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

#[derive(Deserialize)]
pub struct Set {
    pub name: String,
    pub r#type: SetType,
}

#[derive(Deserialize)]
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
