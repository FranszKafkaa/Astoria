use tokio::sync::mpsc;

use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::{ClientConfig, Message};

pub struct KafkaConsumer {
    consumer: StreamConsumer,
}

impl KafkaConsumer {
    pub fn new() -> Result<Self, ()> {
        let consumer: StreamConsumer = ClientConfig::new()
            .set("group.id", "group_id")
            .set("bootstrap.servers", "localhost:9092")
            .set("enable.partition.eof", "false")
            .create()
            .expect("erro ao criar consumer");

        Ok(KafkaConsumer { consumer })
    }

    pub async fn run_async_processor(&self, tx: mpsc::Sender<String>) {
        self.consumer.subscribe(&["video"]).expect("erro");

        loop {
            match self.consumer.recv().await {
                Ok(msg) => {
                    if let Some(Ok(payload)) = msg.payload_view::<str>() {
                        if let Err(_) = tx.send(payload.to_string()).await {
                            eprintln!("Failed to send message to processing task");
                        }
                    }
                }

                Err(err) => eprintln!("{:?} ai que gay", err),
            }
        }
    }
}
