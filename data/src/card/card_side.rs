use serde::Deserialize;

use crate::card::{Keyword, Resource, SideSchemeIcon, Trait};

#[derive(Deserialize)]
pub struct CardSide {
    pub name: String,
    pub text: String,
    pub flavor_text: Option<String>,
    pub illustrators: Option<Vec<String>>,
    #[serde(flatten)]
    pub variant: CardSideVariant,
}

#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum CardSideVariant {
    Hero {
        side: Side,
        #[serde(default)]
        unique: bool,
        thw: String,
        atk: String,
        def: String,
        hand_size: u32,
        hit_points: String,
        #[serde(default)]
        traits: Vec<Trait>,
    },
    #[serde(rename = "Alter-Ego")]
    AterEgo {
        side: String,
        #[serde(default)]
        unique: bool,
        rec: String,
        hand_size: u32,
        hit_points: String,
        #[serde(default)]
        traits: Vec<Trait>,
    },
    Ally {
        subname: Option<String>,
        unique: bool,
        cost: String,
        thw: String,
        thw_consequential: u32,
        atk: String,
        atk_consequential: u32,
        hit_points: String,
        traits: Vec<Trait>,
        resources: Vec<Resource>,
    },
    Event {
        cost: String,
        #[serde(default)]
        traits: Vec<Trait>,
        resources: Vec<Resource>,
    },
    Support {
        cost: String,
        #[serde(default)]
        unique: bool,
        traits: Vec<Trait>,
        resources: Vec<Resource>,
    },
    #[serde(rename = "Side Scheme")]
    SideScheme {
        icons: Option<Vec<SideSchemeIcon>>,
        starting_threat: String,
        boost_icons: u8,
        #[serde(default)]
        star_icon: bool,
    },
    Minion {
        unique: bool,
        sch: String,
        atk: String,
        hit_points: String,
        boost_icons: u8,
        #[serde(default)]
        boost_star_icon: bool,
        keywords: Vec<Keyword>,
    },
}

#[derive(Deserialize)]
pub enum Side {
    A,
    B,
    // support Ant-Man/Wasp giant cards
    C,
}
