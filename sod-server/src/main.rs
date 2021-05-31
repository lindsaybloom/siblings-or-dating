mod error;
mod filters;
mod handlers;
mod models;

use self::models::Post;
use mongodb::{
    bson::doc,
    options::{ClientOptions, ResolverConfig},
    Client, Collection, Database,
};
use std::env;
use warp::Filter;

type Db = Database;
type Error = error::Error;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client_uri = env::var("MONGODB_URI").unwrap_or("mongodb://localhost:27017".to_string());

    let options =
        ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
            .await?;
    let client = Client::with_options(options)?;

    let db = client.database("sod");

    let api = filters::posts(db);

    let routes = api.with(warp::log("posts"));
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;

    Ok(())
}
