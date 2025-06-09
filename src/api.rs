use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Serialize, Deserialize)]
pub struct UserAction {
    pub user_id: u32,
    pub action: String,
    pub producer_id: String,
}

async fn send_action(action: web::Json<UserAction>) -> impl Responder {
    // Configure the Kafka producer
    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", "localhost:9092")
        .set("message.timeout.ms", "5000")
        .create()
        .expect("Producer creation failed");

    let topic = "user-actions";

    let payload = serde_json::to_string(&action).expect("Serialization failed");
    let user_id =action.user_id.to_string();
    let record = FutureRecord::to(topic)
        .payload(&payload)
        .key(&user_id);

    println!("[API Producer] Sending action: user_id={}, action={}, producer_id={}", 
        action.user_id, action.action, action.producer_id);

    match producer.send(record, Duration::from_secs(0)).await {
        Ok(delivery) => {
            println!("[API Producer] Successfully sent action for user_id={} to partition={}", 
                action.user_id, delivery.1);
            HttpResponse::Ok().json("Action sent to Kafka")
        }
        Err(e) => {
            println!("[API Producer] Failed to send action: {:?}", e);
            HttpResponse::InternalServerError().json(format!("Failed to send action: {:?}", e))
        }
    }
}

pub async fn run_api_server() {
    println!("[API Server] Starting on http://localhost:8080");
    HttpServer::new(|| {
        App::new()
            .route("/action", web::post().to(send_action))
    })
    .bind("127.0.0.1:8080")
    .expect("Failed to bind to port 8080")
    .run()
    .await
    .expect("API server failed");
}