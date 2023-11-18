use warp::{Filter, Rejection, Reply};
use serde::{Deserialize, Serialize};



async fn user(id: String) -> Result<impl Reply, Rejection> {
    Ok(format!("Got {}", id))
}

#[tokio::main]
async fn main() {
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let main = warp::path::end()
        .map(|| warp::reply::html("no"));

    let user = warp::get()
        .and(warp::path("user"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and_then(user);

    main.or(user);

    warp::serve(user)
        .run(([127, 0, 0, 1], 3030))
        .await;
}