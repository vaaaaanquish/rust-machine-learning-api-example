use axum::{handler::post, Router, Json, AddExtensionLayer, extract::Extension};
use serde::{Serialize, Deserialize};
use serde_json::{json, Value};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::Mutex;
use tch::nn::ModuleT;
use tch::vision::{resnet, imagenet};

extern crate tch;
extern crate base64;
extern crate image;

struct DnnModel {
    net: Mutex<Box<dyn ModuleT>>
}

#[tokio::main]
async fn main() {
    let weights = std::path::Path::new("/resnet18.ot"); 
    let mut vs = tch::nn::VarStore::new(tch::Device::Cpu);
    let net:Mutex<Box<(dyn ModuleT + 'static)>> = Mutex::new(Box::new(resnet::resnet18(&vs.root(), imagenet::CLASS_COUNT)));
    let _ = vs.load(weights);
    let state = Arc::new(DnnModel { net });

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
    img: String,
}

#[derive(Serialize)]
struct ResponseJson {
    result: Vec<String>,
}

async fn proc(Json(payload): Json<RequestJson>, Extension(state): Extension<Arc<DnnModel>>) -> Json<Value> {
    let net = state.net.lock().await;

    let img_buffer = base64::decode(&payload.img).unwrap();
    let img = image::load_from_memory(&img_buffer.as_slice()).unwrap();

    // to use load_image_and_resize224_from_memory next version tch-rs
    let _ = img.save("/tmp.jpeg");
    let img_tensor = imagenet::load_image_and_resize224("/tmp.jpeg").unwrap();
    let output = net
        .forward_t(&img_tensor.unsqueeze(0), false)
        .softmax(-1, tch::Kind::Float);

    let mut result = Vec::new();
    for (probability, class) in imagenet::top(&output, 5).iter() {
        result.push(format!("{:50} {:5.2}%", class, 100.0 * probability).to_string());
    }
    Json(json!({ "result": result }))
}

