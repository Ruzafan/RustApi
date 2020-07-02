use warp::Filter;

mod User;
mod common;
mod mongo_repository;

#[tokio::main]
async fn main() {
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    //let hello = warp::path!("hello" / String).map(|name| format!("Hello, {}!", name));

    let add_user = warp::path("add_user")
        .and(warp::path::param())
        .and(warp::path::end())
        .and_then(|param: String| mongo_repository::insert_user(param));

    let get_user = warp::path("get_user")
        .and(warp::path::param())
        .and(warp::path::end())
        .and_then(|param: String| mongo_repository::get_user(param));

    let routes = add_user.or(get_user);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
