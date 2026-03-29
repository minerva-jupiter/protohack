use tokio::{
    self,
    io::{AsyncReadExt, AsyncWriteExt},
};
#[tokio::main]
async fn main() -> std::io::Result<()> {
    println!("Smoketest server has started");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    loop {
        match listener.accept().await {
            Ok((mut socket, _)) => {
                let mut buf = Vec::with_capacity(4096);
                socket.read_buf(&mut buf).await?;
                println!("{:?}", buf);
                socket.write_all(&buf).await?;
            }
            Err(e) => {
                return Err(e.into());
            }
        }
    }
}
