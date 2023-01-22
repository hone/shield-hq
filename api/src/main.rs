use axum::{
    extract::{FromRef, State},
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::{get, post},
    Json, Router,
};
use data::graphql::{Ctx, Query, SHQScalarValue, Schema};
use juniper::{http::GraphQLRequest, EmptyMutation, EmptySubscription, RootNode};
use std::{net::SocketAddr, sync::Arc};
use tracing::info;

async fn graphiql() -> impl IntoResponse {
    Html(juniper::http::graphiql::graphiql_source("/graphql", None))
}

async fn graphql(
    State(schema): State<Arc<Schema>>,
    State(ctx): State<Arc<Ctx>>,
    req: Json<GraphQLRequest<SHQScalarValue>>,
) -> impl IntoResponse {
    info!("{:?}", req);
    let response = req.execute(&schema, &ctx).await;
    let status = if response.is_ok() {
        StatusCode::OK
    } else {
        StatusCode::BAD_REQUEST
    };
    let json = serde_json::to_string(&response).unwrap();

    (status, json)
}

#[derive(Clone, FromRef)]
struct AppState {
    schema: Arc<Schema>,
    ctx: Arc<Ctx>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let schema = Arc::new(RootNode::new_with_scalar_value(
        Query {},
        EmptyMutation::<Ctx>::new(),
        EmptySubscription::<Ctx>::new(),
    ));
    let cards_doc: data::card::Document =
        toml::from_str(include_str!("../../data/data/core-set.toml")).unwrap();
    let products_doc: data::product::Document =
        toml::from_str(include_str!("../../data/data/products.toml")).unwrap();
    let ctx = Arc::new(Ctx::new(cards_doc.cards, products_doc.products));
    let state = AppState { schema, ctx };

    let app = Router::new()
        .route("/", get(graphiql))
        .route("/graphql", post(graphql))
        .with_state(state);
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
