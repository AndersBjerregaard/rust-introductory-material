pub mod arcmutexhandles;

use std::{sync::Arc, time::Duration};

use lapin::{
    message::DeliveryResult, options::{BasicConsumeOptions, BasicAckOptions}, types::FieldTable, Channel, Connection,
    ConnectionProperties,
};

#[tokio::main]
async fn main() {
    let shared_data = Arc::new(tokio::sync::Mutex::new(0));

    let uri = "amqp://guest:guest@localhost:5673/%2F";

    let options = ConnectionProperties::default()
        .with_connection_name("load-balancer-connection".to_string().into())
        .with_executor(tokio_executor_trait::Tokio::current())
        .with_reactor(tokio_reactor_trait::Tokio);

    let connection = connect_rabbitmq(uri, options).await;

    let channel = connection.create_channel().await.unwrap();

    let consumer = channel
        .basic_consume(
            "hello",
            "my_consumer",
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await
        .unwrap();

    consumer.set_delegate({
        // Clone the channel so we can move the new instance into the
        // closure and the rest of the code can still use the
        // outer channel variable
        let channel = channel.clone();

        let shared_data = Arc::clone(&shared_data);

        // Return the closure from this block
        move |delivery: DeliveryResult| {
            // Create another clone of the channel
            // that can be moved into the future returned by the async move block
            let channel = channel.clone();

            let shared_data = Arc::clone(&shared_data);
            async move {
                // Do something with channel before dropping
                drop(channel);
                let delivery = match delivery {
                    Ok(Some(delivery)) => delivery,
                    // The consumer got canceled
                    Ok(None) => return,
                    // Carries the error and is always followed by Ok(None)
                    Err(error) => {
                        dbg!("Failed to consume queue message {}", error);
                        return;
                    }
                };

                let mut data = shared_data.lock().await;
                *data += 1;

                // Do something with the delivery data (The message payload)

                delivery
                    .ack(BasicAckOptions::default())
                    .await
                    .expect("Failed to ack send_webhook_event message");
            }
        }
    });
}

async fn connect_rabbitmq(uri: &str, con_props: ConnectionProperties) -> Connection {
    let mut connection = Connection::connect(uri, con_props.clone()).await;

    while connection.is_err() {
        println!(
            "--> Failed to connect to rabbitmq: {}",
            &connection.unwrap_err()
        );
        println!("--> Attempting to reconnect in 3 seconds...");
        std::thread::sleep(Duration::from_millis(3000));
        connection = Connection::connect(uri, con_props.clone()).await;
    }
    println!("--> Connected to rabbitmq!");
    // There might be a way to register a callback to handle dropped connections here
    let connection = connection.unwrap();
    connection
}
