use warp::Filter;

mod mongo_repository;

#[tokio::main]
async fn main() {
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = warp::path!("hello" / String).map(|name| format!("Hello, {}!", name));

    let get_collection = warp::get()
        .and(warp::path("get_collections"))
        .and(warp::path::end())
        .and_then(mongo_repository::init_mongo);

    warp::serve(get_collection)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
