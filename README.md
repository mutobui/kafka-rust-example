# kafka-rust-example

A Rust-based microservices project using Apache Kafka for event-driven communication. This example includes a producer sending random user actions, two consumers processing them, and a placeholder API server.

## Setup Docker

1. **Install Docker**:
   - Download and install Docker Desktop: https://www.docker.com/products/docker-desktop
   - Ensure Docker is running.

2. **Create `docker-compose.yml`**:
   Save the following in `docker-compose.yml`:
   ```yaml
   version: '3'
   services:
     kafka:
       image: confluentinc/cp-kafka:7.7.0
       ports:
         - 9092:9092
         - 9093:9093
       environment:
         KAFKA_BROKER_ID: 1
         KAFKA_PROCESS_ROLES: broker,controller
         KAFKA_NODE_ID: 1
         KAFKA_LISTENER_SECURITY_PROTOCOL_MAP: PLAINTEXT:PLAINTEXT,CONTROLLER:PLAINTEXT
         KAFKA_LISTENERS: PLAINTEXT://0.0.0.0:9092,CONTROLLER://0.0.0.0:9093
         KAFKA_ADVERTISED_LISTENERS: PLAINTEXT://localhost:9092
         KAFKA_CONTROLLER_QUORUM_VOTERS: 1@localhost:9093
         KAFKA_CONTROLLER_LISTENER_NAMES: CONTROLLER
         KAFKA_GROUP_INITIAL_REBALANCE_DELAY_MS: 0
         KAFKA_OFFSETS_TOPIC_REPLICATION_FACTOR: 1
         KAFKA_TRANSACTION_STATE_LOG_REPLICATION_FACTOR: 1
         KAFKA_TRANSACTION_STATE_LOG_MIN_ISR: 1
         CLUSTER_ID: MkU3OEVBNTcwNTJENDM2Qk
       volumes:
         - kafka_data:/var/lib/kafka/data
   volumes:
     kafka_data:
   ```

3. **Start Kafka**:
   ```bash
   docker-compose up -d
   ```

4. **Create the `user-actions` Topic**:
   ```bash
   docker exec -it <kafka-container-id> kafka-topics --create --topic user-actions --bootstrap-server localhost:9092 --partitions 1 --replication-factor 1
   ```
   Find `<kafka-container-id>` with `docker ps`.

5. **Verify Kafka**:
   ```bash
   docker-compose ps
   ```
   Ensure the Kafka container is running.

## Run the Program

1. **Install Rust**:
   - Install Rust via `rustup`: https://www.rust-lang.org/tools/install

2. **Install Dependencies**:
   Ensure `librdkafka` is installed:
   - **macOS**:
     ```bash
     brew install librdkafka
     ```
   - **Ubuntu**:
     ```bash
     sudo apt-get install librdkafka-dev
     ```
   - **Windows**: Follow `rust-rdkafka` instructions: https://github.com/fede1024/rust-rdkafka#windows

3. **Run the Application**:
   ```bash
   cargo run
   ```
   **What Happens**:
   - A producer sends random user actions (e.g., `{ user_id: 45, action: "click" }`) to the `user-actions` topic every second.
   - Two consumers (`analytics` and `purchase`) in separate consumer groups receive and print each message.
   - A placeholder API server runs (replace with your actual server in `api.rs`).
   - Example output:
     ```
     API server running (placeholder)
     Consumer 'analytics' started
     Consumer 'purchase' started
     Sent action: UserAction { user_id: 45, action: "click" }
     Consumer 'analytics': Received action: user_id=45, action=click
     Consumer 'purchase': Received action: user_id=45, action=click
     ...
     ```

4. **Stop the Program**:
   Press `Ctrl+C` to shut down gracefully:
   ```
   Received Ctrl+C, shutting down...
   Producer shutting down
   Consumer 'analytics' shutting down
   Consumer 'purchase' shutting down
   API server shutting down
   All threads shut down gracefully
   ```

## Run More Consumers and See What Happens

1. **Modify `main.rs` to Add a Consumer**:
   Open `src/main.rs` and add another consumer thread. For example, add this before the `// Handle Ctrl+C` line:
   ```rust
   let consumer3_shutdown_rx = shutdown_rx.clone();
   let consumer3_handle = thread::spawn(move || {
       let rt = tokio::runtime::Runtime::new().unwrap();
       rt.block_on(consumer::run_consumer("extra", "extra-group", consumer3_shutdown_rx));
   });
   ```
   Add `consumer3_handle.join().expect("Consumer3 thread panicked");` before `println!("All threads shut down gracefully");`.

2. **Run the Program Again**:
   ```bash
   cargo run
   ```

3. **What Happens**:
   - A new consumer (`extra`) in a new consumer group (`extra-group`) starts.
   - It receives the same messages as `analytics` and `purchase`, since each consumer group gets a full copy of the topic’s messages.
   - Example output:
     ```
     Consumer 'extra' started
     Sent action: UserAction { user_id: 72, action: "view" }
     Consumer 'analytics': Received action: user_id=72, action=view
     Consumer 'purchase': Received action: user_id=72, action=view
     Consumer 'extra': Received action: user_id=72, action=view
     ...
     ```
   - **Why?** Each consumer group subscribes to all partitions of `user-actions` (currently 1 partition), so all groups receive every message. If you had multiple partitions and consumers in the same group, they would split the partitions for load balancing.

4. **Experiment**:
   - Add a consumer to an existing group (e.g., change `"extra-group"` to `"analytics-group"`) and observe load balancing (with multiple partitions).
   - Increase partitions in `user-actions`:
     ```bash
     docker exec -it <kafka-container-id> kafka-topics --alter --topic user-actions --partitions 2 --bootstrap-server localhost:9092
     ```
   - Run multiple consumers in the same group to see them share messages.

## Cleanup

Stop and remove Kafka:
```bash
docker-compose down
docker volume rm kafka-rust-example_kafka_data
```# kafka-rust-example
