use serde::Deserialize;

#[derive(Deserialize)]
pub enum Trait {
    Aerial,
    Attack,
    Avenger,
    Criminal,
    Defense,
    Genius,
    #[serde(rename = "Hero for Hire")]
    HeroForHire,
    Persona,
    Skill,
    #[serde(rename = "S.H.I.E.L.D.")]
    Shield,
    Spy,
    Superpower,
}
