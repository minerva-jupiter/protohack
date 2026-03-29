#[tokio::main]
async fn main() -> std::io::Result<()> {
    println!("Smoketest server has started");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    loop {
        let (mut socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            let (mut rd, mut wr) = socket.split();
            if let Err(e) = tokio::io::copy(&mut rd, &mut wr).await {
                println!("echo error: {}", e);
            }
        });
    }
}
