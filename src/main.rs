use futures_util::StreamExt;
use glam::Vec2;
use serde::{Deserialize, Serialize};
// use tokio_stream::StreamExt;
use tokio::sync::mpsc;
use warp::ws::{Message, WebSocket};
use warp::Filter;

#[derive(Clone, Deserialize, Serialize)]
pub struct State {
    pub pos: Vec2,
    pub r: f32,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct RemoteState {
    pub id: usize,
    pub position: Vec2,
    pub rotation: f32,
}

#[derive(Deserialize, Serialize)]
pub enum ServerMessage {
    Welcome(usize),
    GoodBye(usize),
    Update(Vec<RemoteState>),
}

#[derive(Deserialize, Serialize)]
pub enum ClientMessage {
    State(State),
}

async fn user_connected(ws: WebSocket) {
    use futures_util::StreamExt;

    let (ws_sender, mut ws_receiver) = ws.split();

    let send_channel = create_send_channel(ws_sender);

    let my_id = send_welcome(&send_channel).await;

    log::debug!("new user connected: {}", my_id);

    while let Some(result) = ws_receiver.next().await {
        let msg = match result {
            Ok(msg) => msg,
            Err(e) => {
                log::warn!("websocket receive error: '{};", e);
                break;
            }
        };
        log::debug!("user sent message: {:?}", msg);
    }

    log::debug!("user disconnected: {}", my_id);
}

fn create_send_channel(ws_sender: futures_util::stream::SplitSink<WebSocket, Message>) -> () {
    unimplemented!()
}

async fn send_welcome(out: &()) -> usize {
    unimplemented!()
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let game = warp::path("game")
        .and(warp::ws())
        .map(|ws: warp::ws::Ws| ws.on_upgrade(move |socket| user_connected(socket)));

    let status = warp::path!("status").map(move || warp::reply::html("Hello"));

    let routes = status.or(game);
    warp::serve(routes).run(([0, 0, 0, 0], 3030)).await
}

async fn send_msg(msg: ServerMessage) {
    let buffer = serde_json::to_vec(&msg).unwrap();
}
