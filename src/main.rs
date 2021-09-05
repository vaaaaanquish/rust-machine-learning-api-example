use axum::{handler::post, Router, Json, AddExtensionLayer, extract::Extension};
use serde::{Serialize, Deserialize};
use serde_json::{json, Value};
use std::net::SocketAddr;
use image::imageops::rotate180;
use std::sync::Arc;
use tokio::sync::Mutex;
use std::collections::HashSet;

extern crate base64;
extern crate image;

struct DataState {
    set: Mutex<HashSet<String>>
}

#[tokio::main]
async fn main() {
    let set = Mutex::new(HashSet::new());
    let state = Arc::new(DataState { set });

    let app = Router::new()
        .route("/", post(proc))
        .layer(AddExtensionLayer::new(state));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Deserialize)]
struct RequestJson {
    name: String,
    img: String,
}

#[derive(Serialize)]
struct ResponseJson {
    result: String,
}

async fn proc(Json(payload): Json<RequestJson>, Extension(state): Extension<Arc<DataState>>) -> Json<Value> {
    let img_buffer = base64::decode(&payload.img).unwrap();
    let mut set = state.set.lock().await;

    let result;
    if set.contains(&payload.name) {
        result = "skip by duplicated";
    } else {
        let img = image::load_from_memory(&img_buffer.as_slice()).unwrap();
        let rotate_img = rotate180(&img);
        rotate_img.save(&payload.name).unwrap();
        set.insert(payload.name);
        result = "saved output image";
    }
    Json(json!({ "result": result }))
}

