use std::sync::Mutex;

#[allow(dead_code)]

use actix_web::{get, post, web, App, HttpServer, Result, Responder};
use actix_web::web::Json;
use rustchain::Blockchain;
use serde::Deserialize;
mod blockchain;

struct AppState {
    blockchain: Mutex<Blockchain>
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let blockchain = web::Data::new(AppState {
        blockchain: Mutex::new(Blockchain::new())
    });
    HttpServer::new(move || {
        App::new()
            .app_data(blockchain.clone())
            .service(blocks)
            .service(mine)
    })
    .bind("127.0.0.1:3001")?
    .run()
    .await
}

#[get("/blocks")]
async fn blocks(data: web::Data<AppState>) -> Result<impl Responder> {
    let blockchain = data.blockchain.lock().unwrap();
    let blocks = blockchain.blocks.clone();
    Ok(Json(blocks))
}

#[derive(Deserialize)]
struct MineRequest {
    data: String
}

#[post("/mine")]
async fn mine(request: Json<MineRequest>, data: web::Data<AppState>) -> Result<impl Responder> {
    let mut blockchain = data.blockchain.lock().unwrap();
    blockchain.add_new_block(&request.data);
    let b = blockchain.blocks.clone();
    Ok(Json(b))
}
