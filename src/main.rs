mod api;
mod consumer;
mod models;
mod producer;   

use std::thread;

#[tokio::main]
async fn main() {
    // Spawn API server thread
    let api_handle = thread::spawn(|| {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(api::run_api_server());
    });

    // Spawn one web-app producer
    let producer_handle = thread::spawn(|| {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(producer::run_producer("web-app"));
    });

    // Spawn two consumers
    let consumer1_handle = thread::spawn(|| {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(consumer::run_consumer("analytics", "analytics-group"));
    });

    let consumer2_handle = thread::spawn(|| {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(consumer::run_consumer("purchase", "purchase-group"));
    });

    // Wait for threads (they run indefinitely)
    api_handle.join().unwrap();
    producer_handle.join().unwrap();
    consumer1_handle.join().unwrap();
    consumer2_handle.join().unwrap();
}