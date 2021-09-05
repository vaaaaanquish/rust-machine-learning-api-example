use axum::{handler::post, Router, Json};
use serde::{Serialize, Deserialize};
use serde_json::{json, Value};
use std::net::SocketAddr;
use image::imageops::rotate180;

extern crate base64;
extern crate image;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", post(proc));
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Deserialize)]
struct RequestJson {
    img: String,
}

#[derive(Serialize)]
struct ResponseJson {
    result: String,
}

async fn proc(Json(payload): Json<RequestJson>) -> Json<Value> {
    let img_buffer = base64::decode(&payload.img).unwrap();
    let img = image::load_from_memory(img_buffer.as_slice()).unwrap();
    let rotate_img = rotate180(&img);
    rotate_img.save("output.png").unwrap();
    Json(json!({ "result": "saved output.png" }))
}
