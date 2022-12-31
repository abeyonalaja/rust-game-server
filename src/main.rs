use warp::Filter;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let status = warp::path!("status").map(move || warp::reply::html("Hello"));
    warp::serve(status).run(([0, 0, 0, 0], 3030)).await
}
