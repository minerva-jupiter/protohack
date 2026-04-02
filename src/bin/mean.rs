use std::collections::BTreeMap;
use tokio::io::{AsyncReadExt, AsyncWriteExt, stdin};
use tokio::sync::oneshot;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let (tx, rx) = oneshot::channel();

    tokio::spawn(async move {
        let mut reader = stdin();
        let mut buffer = [0u8; 1];
        loop {
            if reader.read_exact(&mut buffer).await.is_ok() {
                if buffer[0] == b'q' {
                    let _ = tx.send(());
                    break;
                }
            }
        }
    });

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    println!("Server started. Press 'q' then Enter to stop.");

    tokio::select! {
        res = async {
            loop {
                let (mut socket, _) = listener.accept().await?;
                tokio::spawn(async move {
                    let (mut rd, mut wr) = socket.split();
                    let mut buf = [0u8; 9];
                    let mut history = BTreeMap::new();
                    loop {
                        if rd.read_exact(&mut buf).await.is_err() {
                            break;
                        }
                        match buf[0] {
                            b'I' => {
                                let t = i32::from_be_bytes(buf[1..5].try_into().unwrap());
                                let p = i32::from_be_bytes(buf[5..9].try_into().unwrap());
                                history.insert(t, p);
                            }
                            b'Q' => {
                                let min = i32::from_be_bytes(buf[1..5].try_into().unwrap());
                                let max = i32::from_be_bytes(buf[5..9].try_into().unwrap());
                                let avg = calculate_average(&history, min, max);
                                if wr.write_all(&avg.to_be_bytes()).await.is_err() {
                                    break;
                                }
                            }
                            _ => break,
                        }
                    }
                });
            }
            #[allow(unreachable_code)]
            Ok::<(), std::io::Error>(())
        } => {
            if let Err(e) = res {
                eprintln!("Accept error: {}", e);
            }
        },
        _ = rx => {
            println!("Shutdown signal received. Exiting...");
        }
    }

    Ok(())
}

fn calculate_average(history: &BTreeMap<i32, i32>, min: i32, max: i32) -> i32 {
    if min > max {
        return 0;
    }
    let subset: Vec<_> = history.range(min..=max).map(|(_, &v)| v as i64).collect();
    if subset.is_empty() {
        return 0;
    }
    (subset.iter().sum::<i64>() / subset.len() as i64) as i32
}
