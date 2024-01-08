use aws_config::load_defaults;
use aws_sdk_s3 as s3;
use anyhow::{Result, Ok};

#[tokio::main]
async fn main() -> Result<()> {
    let config = load_defaults(aws_config::BehaviorVersion::v2023_11_09()).await;
    let client = s3::Client::new(&config);
    let binding = client.list_buckets().send().await?;
    let buckets = binding.buckets();
    println!("buckets: {:?}", buckets);
    Ok(())
}
