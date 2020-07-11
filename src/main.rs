use warp::Filter;
use login::Login;
use user::UserData;

#[path = "./entities/user.rs"]
mod user;
#[path = "./entities/login.rs"]
mod login;
mod common;
mod mongo_repository;



#[tokio::main]
async fn main() {

    let add_user = warp::post()
        .and(warp::path("add_user"))
        // Only accept bodies smaller than 16kb...
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::json())
        .and_then(|user: UserData| {
            mongo_repository::insert_user(user)
        });

        let get_user = warp::path("get_user")
        .and(warp::path::param())
        .and(warp::path::end())
        .and_then(|param: String| mongo_repository::get_user(param));

        let login_request = warp::post()
        .and(warp::path("login"))
        // Only accept bodies smaller than 16kb...
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::json::<Login>())
        .and(warp::path::end())
        .and_then(|login: Login| {
            mongo_repository::login(login.email, login.password)
        });
     

   
    let routes = add_user
        .or(get_user)
        .or(login_request);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
