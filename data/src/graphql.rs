use crate::{
    card::Card,
    product::{Product, ProductType},
};
use chrono::NaiveDate;
use juniper::{graphql_object, Context, EmptyMutation, EmptySubscription, FieldResult, RootNode};

mod scalar;
pub use scalar::SHQScalarValue;

/// Macro to simplify writing graphql filters
macro_rules! filter {
    ( $filter:ident, $($item:expr => $input:ident,)+ ) => {
        $(
            if let Some($input) = &$input {
                $filter = $item == $input && $filter;
            }
        )*
    };
}
pub(crate) use filter;

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
                    &product.wave => wave,
                );

                filter
            })
            .collect())
    }

    fn all_cards(context: &Ctx) -> FieldResult<&Vec<Card>> {
        Ok(&context.cards)
    }
}

pub type Schema =
    RootNode<'static, Query, EmptyMutation<Ctx>, EmptySubscription<Ctx>, SHQScalarValue>;
