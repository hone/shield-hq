use juniper::GraphQLEnum;
use serde::Deserialize;

#[derive(Deserialize, GraphQLEnum)]
pub enum Trait {
    Aerial,
    Armor,
    Attack,
    Attorney,
    Avenger,
    Brute,
    Condition,
    Criminal,
    Defense,
    Elite,
    Gamma,
    Genius,
    #[serde(rename = "Hero for Hire")]
    HeroForHire,
    Item,
    Location,
    Persona,
    Skill,
    #[serde(rename = "S.H.I.E.L.D.")]
    Shield,
    Soldier,
    Spy,
    Superpower,
    Tech,
    Thwart,
}
