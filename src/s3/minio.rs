use std::{
    fs::File,
    io::Write,
};

use rusoto_core::{HttpClient, Region};
use rusoto_credential::StaticProvider;
use rusoto_s3::{GetObjectRequest, S3Client, S3};
use tokio::io::AsyncReadExt;

pub struct Client {
    client: S3Client,
}

impl Client {
    pub fn new() -> Result<Client, ()> {
        let provider = StaticProvider::new(
            std::env::var("S3_ACCESS_KEY").expect("S3_ACCESS_KEY não declarado"),
            std::env::var("S3_SECRET_KEY").expect("S3_SECRET_KEY não declarado"),
            None,
            None,
        );

        let region = Region::Custom {
            name: "local".to_string(),
            endpoint: std::env::var("S3_ENDPOINT").expect("S3_ENDPOINT nao declarado"),
        };

        let client = S3Client::new_with(
            HttpClient::new().expect("Erro"),
            provider.clone(),
            region.clone(),
        );

        let client = Client { client };
        Ok(client)
    }

    pub async fn list_buckets(&mut self) {
        println!("check if the bucket exists");

        match self.client.list_buckets().await {
            Ok(output) => {
                println!("buckets");
                for bucket in output.buckets.unwrap_or_default() {
                    println!(" {:?}", bucket.name.unwrap_or_default());
                }
            }
            Err(err) => {
                eprintln!(" {:?}", err)
            }
        }
    }

    pub async fn get_object(&self) {
        let request = GetObjectRequest {
            bucket: "main".to_string(),
            key: "penis.mp4".to_string(),
            ..Default::default()
        };

        match self.client.get_object(request).await {
            Ok(response) => {
                if let Some(body) = response.body {
                    let mut bytes: Vec<u8> = Vec::new();
                    body.into_async_read()
                        .read_to_end(&mut bytes)
                        .await
                        .unwrap();

                    let mut file = File::create("outros.mp4").expect("erro ao criar arquivo");
                    file.write_all(&bytes).expect("erro ao escrever o arquivo");
                }
            }
            Err(err) => eprintln!("{:?}", err),
        }
    }
}
