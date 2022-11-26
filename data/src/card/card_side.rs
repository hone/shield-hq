use serde::Deserialize;

use crate::card::{BasicPower, Cost, HitPoints, Keyword, Resource, SideSchemeIcon, Trait};

#[derive(Deserialize)]
pub struct CardSide {
    pub name: String,
    pub text: Option<String>,
    pub flavor_text: Option<String>,
    pub illustrators: Option<Vec<String>>,
    #[serde(default)]
    pub traits: Vec<Trait>,
    #[serde(flatten)]
    pub variant: CardSideVariant,
}

#[derive(Deserialize)]
#[serde(tag = "type")]
#[serde(deny_unknown_fields)]
pub enum CardSideVariant {
    Hero {
        side: Side,
        #[serde(default)]
        unique: bool,
        thw: BasicPower,
        atk: BasicPower,
        def: BasicPower,
        hand_size: u32,
        hit_points: HitPoints,
        #[serde(default)]
        traits: Vec<Trait>,
    },
    #[serde(rename = "Alter-Ego")]
    AterEgo {
        side: Side,
        #[serde(default)]
        unique: bool,
        rec: BasicPower,
        hand_size: u32,
        hit_points: HitPoints,
        #[serde(default)]
        traits: Vec<Trait>,
    },
    Ally {
        subname: Option<String>,
        unique: bool,
        cost: Cost,
        thw: BasicPower,
        thw_consequential: u32,
        atk: BasicPower,
        atk_consequential: u32,
        hit_points: HitPoints,
        #[serde(default)]
        traits: Vec<Trait>,
        resources: Vec<Resource>,
    },
    Event {
        cost: Cost,
        #[serde(default)]
        traits: Vec<Trait>,
        resources: Vec<Resource>,
    },
    Obligation {
        #[serde(default)]
        boost_icons: u32,
    },
    Resource {
        resources: Vec<Resource>,
    },
    Support {
        cost: Cost,
        #[serde(default)]
        unique: bool,
        #[serde(default)]
        traits: Vec<Trait>,
        resources: Vec<Resource>,
    },
    Upgrade {
        cost: Cost,
        #[serde(default)]
        unique: bool,
        resources: Vec<Resource>,
    },
    Attachment {
        boost_icons: u8,
    },
    Minion {
        unique: bool,
        sch: BasicPower,
        atk: BasicPower,
        hit_points: HitPoints,
        #[serde(default)]
        traits: Vec<Trait>,
        #[serde(default)]
        boost_icons: u8,
        #[serde(default)]
        boost_star_icon: bool,
        boost_text: Option<String>,
        #[serde(default)]
        keywords: Vec<Keyword>,
    },
    #[serde(rename = "Side Scheme")]
    SideScheme {
        icons: Option<Vec<SideSchemeIcon>>,
        #[serde(default)]
        traits: Vec<Trait>,
        starting_threat: String,
        #[serde(default)]
        boost_icons: u8,
        #[serde(default)]
        boost_star_icon: bool,
        boost_text: Option<String>,
    },
    Treachery {
        #[serde(default)]
        boost_icons: u8,
        #[serde(default)]
        boost_star_icon: bool,
        boost_text: Option<String>,
    },
}

#[derive(Deserialize)]
pub enum Side {
    A,
    B,
    // support Ant-Man/Wasp giant cards
    C,
}
