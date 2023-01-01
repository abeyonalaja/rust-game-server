use glam::Vec2;
use warp::Filter;

#[derive(Clone)]
pub struct State {
    pub pos: Vec2,
    pub r: f32,
}

#[derive(Clone)]
pub struct RemoteState {
    pub id: usize,
    pub position: Vec2,
    pub rotation: f32,
}

pub enum ServerMessage {
    Welcome(usize),
    GoodBye(usize),
    Update(Vec<RemoteState>),
}

pub enum ClientMessage {
    State(State),
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let status = warp::path!("status").map(move || warp::reply::html("Hello"));
    warp::serve(status).run(([0, 0, 0, 0], 3030)).await
}
