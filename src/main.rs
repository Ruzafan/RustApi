use warp::Filter;
mod MongoRepository;

#[tokio::main]
async fn main() {
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = warp::path!("hello" / String)
        .map(|name| format!("Hello, {}!", name));
    
        let get_collection = warp::get()
        .and(warp::path("get_collecions"))
        .and(warp::path::end())
        .and_then(MongoRepository::init_mongo().await);

    warp::serve(hello)
        .run(([127, 0, 0, 1], 3030))
        .await;
}