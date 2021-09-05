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
    let result = check_img(&state, img_buffer, payload.name);
    Json(json!({ "result": result.await }))
}


async fn check_img(state: &DataState, img_buffer: Vec<u8>, name: String) -> &str {
    let mut set = state.set.lock().await;

    let result;
    if set.contains(&name) {
        result = "skip by duplicated";
    } else {
        let img = image::load_from_memory(&img_buffer.as_slice()).unwrap();
        let rotate_img = rotate180(&img);
        rotate_img.save(&name).unwrap();
        set.insert(name);
        result = "saved output image";
    }
    return result
}
