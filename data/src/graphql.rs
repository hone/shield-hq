use crate::{
    card::{Card, CardInput},
    product::{Product, ProductType},
};
use chrono::NaiveDate;
use juniper::{graphql_object, Context, EmptyMutation, EmptySubscription, FieldResult, RootNode};
use std::collections::HashMap;

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

macro_rules! filter_vec {
    ( $filter:ident, $($item:expr => $input:expr), + ) => {
        $(
            if let Some(input) = $input {
                if input.is_some() && $item.is_some() {
                    let a: HashSet<_> = input.as_ref().unwrap().iter().collect();
                    let b: HashSet<_> = $item.unwrap().iter().collect();

                    $filter = $filter && a.intersection(&b).next().is_some();
                } else if input.as_ref() != $item {
                    $filter = false;
                }
            }
        )*
    };
}
pub(crate) use filter_vec;

macro_rules! filter_context {
    ( $filter:ident, $($item:expr => $input:expr),+ ) => {
        $(
            if let Some(input) = &$input {
                if let Some(ctx) = $item {
                    $filter = $filter && ctx == input;
                }
            }
        )*
    };
}
pub(crate) use filter_context;

#[derive(Default)]
pub struct Ctx {
    pub cards: Vec<Card>,
    pub products: Vec<Product>,
    // https://github.com/graphql-rust/juniper/issues/143
    // Just going to clone products again in memory, since I don't want to deal with lifetime
    // parameters in the Context object in juniper
    products_index: HashMap<String, Product>,
}

impl Context for Ctx {}

impl Ctx {
    pub fn new(cards: Vec<Card>, products: Vec<Product>) -> Self {
        let products_index: HashMap<_, _> = products
            .iter()
            .map(|product| (product.code.clone(), product.clone()))
            .collect();

        Self {
            cards,
            products,
            products_index,
        }
    }

    pub fn product(&self, code: impl AsRef<str>) -> Option<&Product> {
        self.products_index.get(code.as_ref())
    }
}

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
                                    products.iter().any(|input_product| {
                                        card_product.included(input_product, context)
                                    })
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
