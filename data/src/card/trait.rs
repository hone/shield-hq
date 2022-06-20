use serde::Deserialize;

#[derive(Deserialize)]
pub enum Trait {
    Aerial,
    Armor,
    Attack,
    Avenger,
    Condition,
    Criminal,
    Defense,
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
