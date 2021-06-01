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
