use reqwest; //cargo add reqwest -> for requests
use futures::future::join_all; //cargo add futures -> various tools for futures

async fn make_request(i: usize) {
    let url = "https://httpbin.org/get";

    println!("Request {}: sending...", i);
    match reqwest::get(url).await {
        Ok(resp) => {
            match resp.text().await {
                Ok(body) => {
                    println!("Request {}: received {} bytes", i, body.len());
                }
                Err(e) => {
                    eprintln!("Request {}: error reading body: {}", i, e);
                }
            }
        }
        Err(e) => {
            eprintln!("Request {}: failed to send: {}", i, e);
        }
    }
}

#[tokio::main] //cargo add tokio --features all -> tokio with async main
async fn main() {
    println!("Launching dummy async requests...");

    let mut tasks = vec![];

    for i in 1..=5 {
        let task = tokio::spawn(make_request(i));
        tasks.push(task);
    }

    let _ = join_all(tasks).await;

    println!("All requests complete.");
}