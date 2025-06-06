mod utils;
use futures::{stream, StreamExt};
use reqwest::Client;
use std::{
    env,
    time::{Duration, Instant},
};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        return Err(utils::Error::CliUsage.into());
    }

    let target = args[1].as_str();

    let http_timeout = Duration::from_secs(10);
    let http_client = Client::builder().timeout(http_timeout).build()?;

    let ports_concurrency = 200;
    let subdomains_concurrency = 100;
    let scan_start = Instant::now();

    let subdomains = utils::enumerate(&http_client, target).await?;

    // Concurrent stream using buffer_unordered + collect
    let scan_result: Vec<utils::Subdomain> = stream::iter(subdomains.into_iter())
        .map(|subdomain| utils::scan_ports(ports_concurrency, subdomain))
        .buffer_unordered(subdomains_concurrency)
        .collect()
        .await;

    // Concurrent stream using an Arc<Mutex<T>>
    // let res: Arc<Mutex<Vec<Subdomain>>> = Arc::new(Mutex::new(Vec::new()));

    // stream::iter(subdomains.into_iter())
    //     .for_each_concurrent(subdomains_concurrency, |subdomain| {
    //         let res = res.clone();
    //         async move {
    //             let subdomain = ports::scan_ports(ports_concurrency, subdomain).await;
    //             res.lock().await.push(subdomain)
    //         }
    //     })
    //     .await;

    let scan_duration = scan_start.elapsed();
    println!("Scan completed in {:?}", scan_duration);

    for subdomain in scan_result {
        println!("{}:", &subdomain.domain);
        for port in &subdomain.open_ports {
            println!("    {}: open", port.port);
        }

        println!();
    }

    Ok(())
}