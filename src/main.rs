mod kafka;
mod s3;

use kafka::consumer::KafkaConsumer;
use s3::minio::Client;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let (tx, mut rx) = mpsc::channel(1);

    let consumer = KafkaConsumer::new().unwrap();

    let mut client = Client::new().unwrap();

    tokio::spawn(async move {
        client.list_buckets().await;
        client.get_object().await;

        consumer.run_async_processor(tx.clone()).await;
    });

    while let Some(msg) = rx.recv().await {
        println!("{:?}", msg);
    }
}
