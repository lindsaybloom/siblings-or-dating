mod error;
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

mod filters {
    use super::*;
    use std::convert::Infallible;
    use warp::{Filter, Rejection};

    pub fn posts(
        db: Db,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        random_post(db)
    }

    /// GET /posts/random
    pub fn random_post(db: Db) -> impl Filter<Extract = impl warp::Reply, Error = Rejection> + Clone {
        warp::path!("posts" / "random")
            .and(warp::get())
            .and(with_db(db))
            .and_then(handlers::random_post)
    }

    fn with_db(db: Db) -> impl Filter<Extract = (Db,), Error = Infallible> + Clone {
        warp::any().map(move || db.clone())
    }
}

mod handlers {
    use super::*;
    use futures::StreamExt;

    pub async fn random_post(db: Db) -> Result<impl warp::Reply, warp::Rejection> {
        let posts: Collection<Post> = db.collection("posts");

        let pipeline = vec![
            // doc! { "$match": { "imgUrl": "ifgkndkn" } },
            doc! { "$sample": { "size" : 1 } },
        ];

        let mut cursor = posts.aggregate(pipeline, None).await.unwrap();

        let result = cursor
            .next()
            .await
            .ok_or(super::Error::NotFound)?
            .map_err(super::Error::from)?;

        Ok(warp::reply::json(&result))
    }
}
