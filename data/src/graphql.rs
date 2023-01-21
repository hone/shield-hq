use crate::{
    card::{Card, CardInput},
    product::{Product, ProductType},
};
use chrono::NaiveDate;
use juniper::{graphql_object, Context, EmptyMutation, EmptySubscription, FieldResult, RootNode};

mod scalar;
pub use scalar::SHQScalarValue;

/// Macro to simplify writing graphql filters
macro_rules! filter {
    ( $filter:ident, $($item:expr => $input:expr),+ ) => {
        $(
            if let Some(input) = &$input {
                $filter = $filter && $item == input;
            }
        )*
    };
}
pub(crate) use filter;

/// Macro to simplify writing graphql filters
macro_rules! filter_option {
    ( $filter:ident, $($item:expr => $input:expr),+ ) => {
        $(
            if let Some(input) = &$input {
                $filter = $filter && $item == input.as_ref();
            }
        )*
    };
}
pub(crate) use filter_option;

pub struct Ctx {
    pub cards: Vec<Card>,
    pub products: Vec<Product>,
}

impl Context for Ctx {}

pub struct Query;

#[graphql_object(Context = Ctx, Scalar = SHQScalarValue)]
impl Query {
    fn products(
        context: &Ctx,
        name: Option<String>,
        release_date: Option<NaiveDate>,
        r#type: Option<ProductType>,
        code: Option<String>,
        wave: Option<u32>,
    ) -> FieldResult<Vec<&Product>> {
        let products = &context.products;

        Ok(products
            .into_iter()
            .filter(|product| {
                let mut filter = true;

                filter!(filter,
                    &product.name => name,
                    &product.release_date => release_date,
                    &product.r#type => r#type,
                    &product.code => code,
                    &product.wave => wave
                );

                filter
            })
            .collect())
    }

    fn cards(context: &Ctx, r#where: Option<CardInput>) -> FieldResult<Vec<&Card>> {
        let cards = &context.cards;

        if let Some(r#where) = r#where {
            Ok(cards
                .into_iter()
                .filter(|card| {
                    let mut filter = true;

                    filter!(filter, &card.aspect => r#where.aspect);

                    if let Some(products) = &r#where.products {
                        filter = filter
                            && !card
                                .products
                                .iter()
                                .filter(|card_product| {
                                    products
                                        .iter()
                                        .any(|input_product| card_product.included(input_product))
                                })
                                .collect::<Vec<_>>()
                                .is_empty();
                    }
                    if let Some(sides) = &r#where.sides {
                        filter = filter
                            && !card
                                .sides
                                .iter()
                                .filter(|card_side| {
                                    sides
                                        .iter()
                                        .any(|input_side| card_side.included(input_side))
                                })
                                .collect::<Vec<_>>()
                                .is_empty();
                    }

                    filter
                })
                .collect())
        } else {
            Ok(cards.into_iter().collect())
        }
    }
}

pub type Schema =
    RootNode<'static, Query, EmptyMutation<Ctx>, EmptySubscription<Ctx>, SHQScalarValue>;
