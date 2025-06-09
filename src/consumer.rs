use crate::models::UserAction;
use rdkafka::config::ClientConfig;
use rdkafka::consumer::{StreamConsumer, Consumer};
use rdkafka::message::Message;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use tokio::time::{interval, Duration};

pub async fn run_consumer(consumer_type: &str, group_id: &str) {
    let action_counts = Arc::new(Mutex::new(HashMap::new()));
    let counts_clone = Arc::clone(&action_counts);

    let consumer_type_owned = consumer_type.to_string();
    tokio::spawn(async move {
        let mut interval = interval(Duration::from_secs(5));
        loop {
            interval.tick().await;
            let counts = counts_clone.lock().unwrap();
            println!("[Consumer {}] Action counts: {:?}", consumer_type_owned, *counts);
        }
    });

    let consumer: StreamConsumer = ClientConfig::new()
        .set("bootstrap.servers", "localhost:9092")
        .set("group.id", group_id)
        .set("auto.offset.reset", "earliest")
        .create()
        .expect("Consumer creation failed");

    consumer.subscribe(&["user-actions"]).expect("Subscription failed");
    println!("[Consumer {}] Subscribed to topic: user-actions", consumer_type);

    loop {
        match consumer.recv().await {
            Ok(msg) => {
                if let Some(payload) = msg.payload() {
                    match serde_json::from_slice::<UserAction>(payload) {
                        Ok(action) => {
                            if consumer_type == "purchase" && action.action != "purchase" {
                                continue;
                            }
                            println!("[Consumer {}] Received action: user_id={}, action={}, producer_id={}, partition={}", 
                                consumer_type, action.user_id, action.action, action.producer_id, msg.partition());
                            let mut counts = action_counts.lock().unwrap();
                            let count = counts.entry(action.user_id).or_insert(0);
                            *count += 1;
                            consumer.commit_message(&msg, rdkafka::consumer::CommitMode::Async).expect("Commit failed");
                        }
                        Err(e) => println!("[Consumer {}] Failed to deserialize message: {:?}", consumer_type, e),
                    }
                } else {
                    println!("[Consumer {}] Received empty payload", consumer_type);
                }
            }
            Err(e) => println!("[Consumer {}] Error receiving message: {:?}", consumer_type, e),
        }
    }
}