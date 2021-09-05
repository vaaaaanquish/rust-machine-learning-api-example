use axum::{handler::post, Router, Json};
use serde::{Serialize, Deserialize};
use serde_json::{json, Value};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new().route("/", post(proc));

    // run it
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Deserialize)]
struct RequestJson {
    message: String,
}

#[derive(Serialize)]
struct ResponseJson {
    message: String,
}

async fn proc(Json(payload): Json<RequestJson>) -> Json<Value> {
    Json(json!({ "message": payload.message + " world!" }))
}
