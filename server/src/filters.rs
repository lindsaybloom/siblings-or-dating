use super::*;
use std::convert::Infallible;
use warp::{Filter, Rejection};

pub fn posts(db: Db) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
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
