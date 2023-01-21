use juniper::GraphQLEnum;
use serde::Deserialize;

#[derive(Clone, Deserialize, GraphQLEnum, Hash, PartialEq, Eq)]
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
    Kree,
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
