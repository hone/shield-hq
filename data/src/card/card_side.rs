use juniper::{graphql_object, GraphQLEnum, GraphQLObject};
use serde::Deserialize;

use crate::card::{BasicPower, Cost, HitPoints, Keyword, Resource, SideSchemeIcon, Trait};

#[derive(Clone, Deserialize, GraphQLObject)]
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

#[derive(Clone, Deserialize)]
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
    AlterEgo {
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
        boost_icons: u8,
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

#[graphql_object]
impl CardSideVariant {
    fn unique(&self) -> Option<&bool> {
        match self {
            CardSideVariant::Hero { unique, .. } => Some(unique),
            CardSideVariant::AlterEgo { unique, .. } => Some(unique),
            CardSideVariant::Ally { unique, .. } => Some(unique),
            CardSideVariant::Support { unique, .. } => Some(unique),
            CardSideVariant::Upgrade { unique, .. } => Some(unique),
            CardSideVariant::Minion { unique, .. } => Some(unique),
            _ => None,
        }
    }

    fn thw(&self) -> Option<&BasicPower> {
        match self {
            CardSideVariant::Hero { thw, .. } => Some(thw),
            CardSideVariant::Ally { thw, .. } => Some(thw),
            _ => None,
        }
    }

    fn atk(&self) -> Option<&BasicPower> {
        match self {
            CardSideVariant::Hero { atk, .. } => Some(atk),
            CardSideVariant::Ally { atk, .. } => Some(atk),
            CardSideVariant::Minion { atk, .. } => Some(atk),
            _ => None,
        }
    }
}

#[derive(Clone, Deserialize, GraphQLEnum)]
pub enum Side {
    A,
    B,
    // support Ant-Man/Wasp giant cards
    C,
}
