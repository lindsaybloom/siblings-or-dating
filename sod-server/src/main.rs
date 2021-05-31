use mongodb::{
    bson::doc,
    options::{ClientOptions, ResolverConfig},
    Client,
};
use std::env;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client_uri = env::var("MONGODB_URI").unwrap_or("mongodb://localhost:27017".to_string());

    let options =
        ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
            .await?;
    let client = Client::with_options(options)?;

    let posts = client.database("sod").collection("posts");

    let new_item = doc! {
        "imgUrl": "ifgkndkn",
        "dating": true,
    };

    let res_id = posts.insert_one(new_item, None).await?;
    println!("new id: {}", res_id.inserted_id);

    let post = posts
        .find_one(doc! { "imgUrl": "ifgkndkn" }, None)
        .await?;

    println!("Post: {}", post.unwrap());

    Ok(())
}
