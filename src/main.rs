// Port of https://www.rabbitmq.com/tutorials/tutorial-one-python.html. Run this
// in one shell, and run the hello_world_publish example in another.
use amiquip::{Connection, ConsumerMessage, ConsumerOptions, QueueDeclareOptions, Result};
use dotenv;
use std::env;
use reqwest::header::ACCEPT;
use serde_json::{from_str, Value};

fn main() -> Result<()> {
    // Load dotenv file
    dotenv::dotenv().ok();

    // Open connection.
    let rabbit_url = format!("{}{}:{}","amqp://guest:guest@",&*env::var("RABBITMQ_ENDPOINT").unwrap(), &*env::var("RABBITMQ_PORT").unwrap());
    let mut connection = Connection::insecure_open(&*rabbit_url)?;

    // Open a channel - None says let the library choose the channel ID.
    let channel = connection.open_channel(None)?;

    // Declare the "hello" queue.
    let queue = channel.queue_declare(&*env::var("RABBITMQ_QUEUE").unwrap(), QueueDeclareOptions::default())?;

    // Start a consumer.
    let consumer = queue.consume(ConsumerOptions::default())?;
    println!("Waiting for messages. Press Ctrl-C to exit.");

    for (i, message) in consumer.receiver().iter().enumerate() {
        match message {
            ConsumerMessage::Delivery(delivery) => {
                let body = String::from_utf8_lossy(&delivery.body);
                println!("({:>3}) Received [{}]", i, body);
                let message = body.into_owned();
                send_post(message);
                consumer.ack(delivery)?;
            }
            other => {
                println!("Consumer ended: {:?}", other);
                break;
            }
        }
    }

    connection.close()
}
#[tokio::main]
async fn send_post(body: String){
    dotenv::dotenv().ok();
    let v: Value = from_str(&*body).unwrap();
    let client = reqwest::Client::new();
    let resp = client
        .post(env::var("AUTOMATE_URL").unwrap())
        .header(ACCEPT,"application/json")
        .json(&v)
        .send()
        .await
        .unwrap();
    let t  = resp
        .text()
        .await
        .unwrap();
    println!("{}", t);

}


