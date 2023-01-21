use crate::{
    card::{BasicPower, Cost, HitPoints, Keyword, Resource, SideSchemeIcon, Trait},
    graphql::{filter, filter_option, SHQScalarValue},
};
use derive_builder::Builder;
use juniper::{graphql_object, GraphQLEnum, GraphQLInputObject};
use serde::Deserialize;
use std::collections::HashSet;

#[derive(Clone, Deserialize)]
pub struct CardSide {
    pub name: String,
    pub text: Option<String>,
    pub flavor_text: Option<String>,
    pub illustrators: Option<Vec<String>>,
    #[serde(flatten)]
    pub variant: CardSideVariant,
}

#[derive(Builder, Clone, GraphQLInputObject)]
#[graphql(scalar = SHQScalarValue)]
pub struct CardSideInput {
    #[builder(default)]
    pub name: Option<String>,
    #[builder(default)]
    pub text: Option<Option<String>>,
    #[builder(default)]
    pub flavor_text: Option<Option<String>>,
    #[builder(default)]
    pub illustrators: Option<Option<Vec<String>>>,
    // Card Side Variant
    #[builder(default)]
    pub unique: Option<Option<bool>>,
    #[builder(default)]
    pub atk: Option<Option<BasicPower>>,
    #[builder(default)]
    pub thw: Option<Option<BasicPower>>,
    #[builder(default)]
    pub def: Option<Option<BasicPower>>,
    #[builder(default)]
    pub hand_size: Option<Option<u32>>,
    #[builder(default)]
    pub hit_points: Option<Option<HitPoints>>,
    #[builder(default)]
    pub traits: Option<Option<Vec<Trait>>>,
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
        #[serde(default)]
        traits: Vec<Trait>,
    },
    Attachment {
        boost_icons: u8,
        #[serde(default)]
        traits: Vec<Trait>,
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

impl CardSide {
    pub fn included(&self, input: &CardSideInput) -> bool {
        let mut filter = true;

        filter!(filter,
            &self.name => input.name,
            &self.text => input.text,
            &self.flavor_text => input.flavor_text
        );
        if let Some(input_illustrators) = &input.illustrators {
            if input_illustrators.is_some() && self.illustrators.is_some() {
                let a: HashSet<_> = input_illustrators.as_ref().unwrap().iter().collect();
                let b: HashSet<_> = self.illustrators.as_ref().unwrap().iter().collect();

                filter = filter && a.intersection(&b).next().is_some();
            } else if input_illustrators.as_ref() != self.illustrators.as_ref() {
                filter = false;
            }
        }

        filter_option!(filter,
            self.unique() => input.unique,
            self.atk() => input.atk,
            self.thw() => input.thw,
            self.def() => input.def,
            self.hand_size() => input.hand_size,
            self.hit_points() => input.hit_points
        );

        if let Some(input_traits) = &input.traits {
            if input_traits.is_some() && self.traits().is_some() {
                let a: HashSet<_> = input_traits.as_ref().unwrap().iter().collect();
                let b: HashSet<_> = self.traits().unwrap().iter().collect();

                filter = filter && a.intersection(&b).next().is_some();
            } else if input_traits.as_ref() != self.traits() {
                filter = false;
            }
        }

        filter
    }

    fn unique(&self) -> Option<&bool> {
        match &self.variant {
            CardSideVariant::Hero { unique, .. } => Some(unique),
            CardSideVariant::AlterEgo { unique, .. } => Some(unique),
            CardSideVariant::Ally { unique, .. } => Some(unique),
            CardSideVariant::Support { unique, .. } => Some(unique),
            CardSideVariant::Upgrade { unique, .. } => Some(unique),
            CardSideVariant::Minion { unique, .. } => Some(unique),
            _ => None,
        }
    }

    fn atk(&self) -> Option<&BasicPower> {
        match &self.variant {
            CardSideVariant::Hero { atk, .. } => Some(atk),
            CardSideVariant::Ally { atk, .. } => Some(atk),
            CardSideVariant::Minion { atk, .. } => Some(atk),
            _ => None,
        }
    }

    fn thw(&self) -> Option<&BasicPower> {
        match &self.variant {
            CardSideVariant::Hero { thw, .. } => Some(thw),
            CardSideVariant::Ally { thw, .. } => Some(thw),
            _ => None,
        }
    }

    fn def(&self) -> Option<&BasicPower> {
        match &self.variant {
            CardSideVariant::Hero { def, .. } => Some(def),
            _ => None,
        }
    }

    fn hand_size(&self) -> Option<&u32> {
        match &self.variant {
            CardSideVariant::Hero { hand_size, .. } => Some(hand_size),
            CardSideVariant::AlterEgo { hand_size, .. } => Some(hand_size),
            _ => None,
        }
    }

    fn hit_points(&self) -> Option<&HitPoints> {
        match &self.variant {
            CardSideVariant::Hero { hit_points, .. } => Some(hit_points),
            CardSideVariant::AlterEgo { hit_points, .. } => Some(hit_points),
            CardSideVariant::Ally { hit_points, .. } => Some(hit_points),
            CardSideVariant::Minion { hit_points, .. } => Some(hit_points),
            _ => None,
        }
    }

    fn traits(&self) -> Option<&Vec<Trait>> {
        match &self.variant {
            CardSideVariant::Hero { traits, .. } => Some(traits),
            CardSideVariant::AlterEgo { traits, .. } => Some(traits),
            CardSideVariant::Ally { traits, .. } => Some(traits),
            CardSideVariant::Event { traits, .. } => Some(traits),
            CardSideVariant::Support { traits, .. } => Some(traits),
            CardSideVariant::Upgrade { traits, .. } => Some(traits),
            CardSideVariant::Attachment { traits, .. } => Some(traits),
            CardSideVariant::Minion { traits, .. } => Some(traits),
            CardSideVariant::SideScheme { traits, .. } => Some(traits),
            _ => None,
        }
    }
}

#[graphql_object(Scalar = SHQScalarValue)]
impl CardSide {
    fn name(&self) -> &String {
        &self.name
    }

    fn text(&self) -> Option<&String> {
        self.text.as_ref()
    }

    fn flavor_text(&self) -> Option<&String> {
        self.flavor_text.as_ref()
    }

    fn illustrators(&self) -> Option<&Vec<String>> {
        self.illustrators.as_ref()
    }

    fn unique(&self) -> Option<&bool> {
        self.unique()
    }

    fn thw(&self) -> Option<&BasicPower> {
        self.thw()
    }

    fn atk(&self) -> Option<&BasicPower> {
        self.atk()
    }

    fn def(&self) -> Option<&BasicPower> {
        self.def()
    }

    fn hand_size(&self) -> Option<&u32> {
        self.hand_size()
    }

    fn hit_points(&self) -> Option<&HitPoints> {
        self.hit_points()
    }

    fn traits(&self) -> Option<&Vec<Trait>> {
        self.traits()
    }
}

#[derive(Clone, Deserialize, GraphQLEnum)]
pub enum Side {
    A,
    B,
    // support Ant-Man/Wasp giant cards
    C,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::Trait;

    fn setup_card_side() -> CardSide {
        CardSide {
            name: String::from("Spider-Man"),
            text: Some(String::from("*Spider-Sense* - **Interrupt**: When the villain activates against you, draw 1 card.")),
            flavor_text: Some(String::from("\"Just your friendly neighborhood Spider-Man!\"")),
            illustrators: None,
            variant: CardSideVariant::Hero {
                side: Side::A,
                unique: true,
                thw: BasicPower::Number(1),
                atk: BasicPower::Number(2),
                def: BasicPower::Number(3),
                hand_size: 5,
                hit_points: HitPoints::Number(10),
                traits: vec![Trait::Avenger],
            },
        }
    }

    #[test]
    fn name_included() {
        let card_side = setup_card_side();
        let input = CardSideInputBuilder::default()
            .name(Some(String::from("Spider-Man")))
            .build()
            .unwrap();

        assert_eq!(true, card_side.included(&input));
    }

    #[test]
    fn name_not_included() {
        let card_side = setup_card_side();
        let input = CardSideInputBuilder::default()
            .name(Some(String::from("Not a Card Side")))
            .build()
            .unwrap();

        assert_eq!(false, card_side.included(&input));
    }

    #[test]
    fn text_included() {
        let card_side = setup_card_side();
        let mut card_side_none = card_side.clone();
        card_side_none.text = None;
        let input = CardSideInputBuilder::default()
            .text(Some(Some(String::from("*Spider-Sense* - **Interrupt**: When the villain activates against you, draw 1 card.")))).build().unwrap();
        let input_none = CardSideInputBuilder::default().text(None).build().unwrap();
        let input_some_none = CardSideInputBuilder::default()
            .text(Some(None))
            .build()
            .unwrap();

        assert_eq!(true, card_side.included(&input));
        assert_eq!(true, card_side.included(&input_none));
        assert_eq!(true, card_side_none.included(&input_some_none));
    }

    #[test]
    fn text_not_included() {
        let card_side = setup_card_side();
        let input = CardSideInputBuilder::default()
            .text(Some(Some(String::from("foo"))))
            .build()
            .unwrap();
        let input_none = CardSideInputBuilder::default()
            .text(Some(None))
            .build()
            .unwrap();

        assert_eq!(false, card_side.included(&input));
        assert_eq!(false, card_side.included(&input_none));
    }

    #[test]
    fn illustrators_included() {
        let card_side = setup_card_side();
        let mut card_side_illustrators = card_side.clone();
        card_side_illustrators.illustrators = Some(vec![
            String::from("Gabriel Eltaeb"),
            String::from("Andrea Di Vito"),
        ]);
        let input = CardSideInputBuilder::default()
            .illustrators(Some(None))
            .build()
            .unwrap();
        let input_none = CardSideInputBuilder::default()
            .illustrators(None)
            .build()
            .unwrap();
        let input2 = CardSideInputBuilder::default()
            .illustrators(Some(Some(vec![String::from("Andrea Di Vito")])))
            .build()
            .unwrap();

        assert_eq!(true, card_side.included(&input));
        assert_eq!(true, card_side.included(&input_none));
        assert_eq!(true, card_side_illustrators.included(&input2));
    }

    #[test]
    fn illustrators_not_included() {
        let mut card_side = setup_card_side();
        card_side.illustrators = Some(vec![String::from("Andrea Di Vito")]);
        let input_none = CardSideInputBuilder::default()
            .illustrators(Some(None))
            .build()
            .unwrap();
        let input = CardSideInputBuilder::default()
            .illustrators(Some(Some(vec![String::from("Peter Parker")])))
            .build()
            .unwrap();

        assert_eq!(false, card_side.included(&input_none));
        assert_eq!(false, card_side.included(&input));
    }
}
