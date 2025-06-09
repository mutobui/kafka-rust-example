use crate::models::UserAction;
use rand::seq::SliceRandom;
use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use std::time::Duration;

pub async fn run_producer(producer_id: &str) {
    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", "localhost:9092")
        .set("message.timeout.ms", "5000")
        .create()
        .expect("Producer creation failed");

    let topic = "user-actions";
    let actions = vec!["click", "view", "purchase", "login"];
    let mut rng = rand::thread_rng();

    loop {
        let action = UserAction {
            user_id: rand::random::<u32>() % 100,
            action: actions.choose(&mut rng).unwrap().to_string(),
            producer_id: producer_id.to_string(),
        };
        let payload = serde_json::to_string(&action).expect("Serialization failed");
        let user_id = action.user_id.to_string();

        println!(
            "[Producer {}] Sending action: user_id={}, action={}",
            producer_id, action.user_id, action.action
        );

        let mut attempts = 0;
        const MAX_RETRIES: u32 = 3;
        loop {
            let record = FutureRecord::to(topic).payload(&payload).key(&user_id);

            match producer.send(record, Duration::from_secs(0)).await {
                Ok(delivery) => {
                    println!(
                        "[Producer {}] Successfully sent action for user_id={} to partition={}",
                        producer_id, action.user_id, delivery.1
                    );
                    break;
                }
                Err(e) => {
                    attempts += 1;
                    if attempts >= MAX_RETRIES {
                        println!(
                            "[Producer {}] Failed to send action after {} attempts: {:?}",
                            producer_id, MAX_RETRIES, e
                        );
                        break;
                    }
                    println!(
                        "[Producer {}] Retry {} for user_id={}: {:?}",
                        producer_id, attempts, action.user_id, e
                    );
                    tokio::time::sleep(Duration::from_secs(1)).await;
                    continue;
                }
            }
        }

        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}
