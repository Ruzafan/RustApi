use warp::Filter;

mod user;
mod common;
mod mongo_repository;


#[tokio::main]
async fn main() {

    let add_user = warp::post()
        .and(warp::path("add_user"))
        // Only accept bodies smaller than 16kb...
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::json())
        .map(|mut user: user::user_data| {
            mongo_repository::insert_user(user.name);
            warp::reply::html("Ok")
        });

        let get_user = warp::path("get_user")
        .and(warp::path::param())
        .and(warp::path::end())
        .and_then(|param: String| mongo_repository::get_user(param));
   
    let routes = add_user.or(get_user);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
