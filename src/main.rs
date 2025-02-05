use tokio::io::{self, AsyncBufReadExt};
use tokio::net::TcpStream;
use tokio_util::codec::{FramedWrite, LinesCodec};
use tokio_stream::StreamExt;

async fn poll_terminal_input() -> io::Result<()> {
    let stdin = io::stdin();
    let mut reader = io::BufReader::new(stdin).lines();

    while let Some(line) = reader.next_line().await? {
        send_to_port(line).await?;
    }

    Ok(())
}

async fn send_to_port(sentence: String) -> io::Result<()> {
    let addr = "127.0.0.1:8000".to_string();
    let stream = TcpStream::connect(addr).await?;
    let mut framed = FramedWrite::new(stream, LinesCodec::new());

    framed.send(sentence).await?;

    Ok(())
}

#[tokio::main]
async fn main() {
    tokio::spawn(async {
        if let Err(e) = poll_terminal_input().await {
            eprintln!("Error polling terminal input: {}", e);
        }
    });

    // Keep the main function running
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
    }
}
