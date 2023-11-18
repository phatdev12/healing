use pocketbase_sdk::{
    admin::Admin,
    client::{Auth, Client},
};
use warp::{Filter, Rejection, Reply};

type PocketBaseDB = Client<Auth>;

async fn user(id: String, db: PocketBaseDB) -> Result<impl Reply, Rejection> {
    let user_collection = db.collections().view(&id).call().unwrap();
    dbg!(&user_collection);
    Ok("hi")
}

#[tokio::main]
async fn main() {
    let authenticated_admin_client = Admin::new("https://healing-pocketbase.tobycm.systems/_/")
        .auth_with_password("tuphat3456tt@gmail.com", "X7JM5r4gkQB-nBk9_3h6rog25Bqm7qJ7")
        .unwrap();
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let main = warp::path::end().map(|| warp::reply::html("no"));

    let user = warp::get()
        .and(warp::path("user"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(with_db(authenticated_admin_client))
        .and_then(user);

    main.or(user);

    warp::serve(main).run(([0, 0, 0, 0], 3030)).await;
}

fn with_db(
    db: PocketBaseDB,
) -> impl Filter<Extract = (PocketBaseDB,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db.clone())
}
