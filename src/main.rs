use anyhow::Result;
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_ec2::Client;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(long, value_parser)]
    instance_id: String,
}

async fn stop_instance(client: &Client, id: &str) -> Result<()> {
    client.stop_instances().instance_ids(id).send().await?;

    println!("Stop instance {}", id);

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let arg = Args::parse();
    let region_provider = RegionProviderChain::default_provider().or_else("ap-northeast-1");
    let config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&config);
    let instance = arg.instance_id.as_str();

    stop_instance(&client, instance).await?;

    Ok(())
}
