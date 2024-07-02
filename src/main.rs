use reqwest::Client;
use std::env;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: cargo run <operation> <file>");
        return;
    }

    let operation = &args[1];
    let file_path = &args[2];

    match operation.as_str() {
        "put-local" => {
            if let Err(e) = put_local(file_path).await {
                eprintln!("Error: {}", e);
            }
        }
        "get-local" => {
            if let Err(e) = get_local(file_path).await {
                eprintln!("Error: {}", e);
            }
        }
        "put-s3" => {
            if let Err(e) = put_s3(file_path).await {
                eprintln!("Error: {}", e);
            }
        }
        "get-s3" => {
            if let Err(e) = get_s3(file_path).await {
                eprintln!("Error: {}", e);
            }
        }
        _ => {
            eprintln!("Unsupported operation: {}", operation);
        }
    }
}

async fn put_local(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let response = client
        .post("http://localhost:8000/put-local")
        .body(file_path.to_string())
        .send()
        .await?;
    println!("Response: {}", response.text().await?);
    Ok(())
}

async fn get_local(file_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let response = client
        .get(format!("http://localhost:8000/get-local/{}", file_name))
        .send()
        .await?;
    println!("Response: {}", response.text().await?);
    Ok(())
}

async fn put_s3(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let response = client
        .post("http://localhost:8000/put-s3")
        .body(file_path.to_string())
        .send()
        .await?;
    println!("Response: {}", response.text().await?);
    Ok(())
}

async fn get_s3(file_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let response = client
        .get(format!("http://localhost:8000/get-s3/{}", file_name))
        .send()
        .await?;
    println!("Response: {}", response.text().await?);
    Ok(())
}
