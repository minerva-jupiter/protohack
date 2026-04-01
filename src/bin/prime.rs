use serde::{Deserialize, Serialize};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt};

#[derive(Deserialize)]
struct PrimeRequest {
    method: String,
    number: f64,
}

#[derive(Serialize)]
struct PrimeResponse {
    method: String,
    prime: bool,
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    println!("Smoketest server has started");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    loop {
        let (mut socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            let (rd, mut wr) = socket.split();
            let mut reader = tokio::io::BufReader::new(rd).lines();
            while let Ok(Some(line)) = reader.next_line().await {
                let request: PrimeRequest = match serde_json::from_str::<PrimeRequest>(&line) {
                    Ok(r) if r.method == "isPrime" => r,
                    _ => continue,
                };
                let is_prime = is_prime(request.number);
                let response = PrimeResponse {
                    method: "is_prime".to_string(),
                    prime: is_prime,
                };
                let mut payload = serde_json::to_vec(&response).unwrap();
                payload.push(b'\n');
                if wr.write_all(&payload).await.is_err() {
                    break;
                }
            }
        });
    }
}

fn is_prime(n: f64) -> bool {
    if n.fract() != 0.0 || n < 2.0 {
        return false;
    }
    for i in 2..(n as u64) {
        if n % i as f64 == 0.0 {
            return false;
        }
    }
    true
}
