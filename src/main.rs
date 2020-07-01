use warp::Filter;

mod common;
mod mongo_repository;

#[tokio::main]

async fn main() {
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    //let hello = warp::path!("hello" / String).map(|name| format!("Hello, {}!", name));

     let add_user = warp::path("add_user")
         .and(warp::path::param())  
         .and(warp::path::end())
         .and_then(|param: String| {
            mongo_repository::insert_user(param)
        });
    warp::serve(add_user)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
