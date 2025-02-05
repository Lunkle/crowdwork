use tokio;
use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut reader = io::BufReader::new(stdin);
    let mut buffer = String::new();

    loop {
        buffer.clear();
        let bytes_read = reader.read_line(&mut buffer).await.unwrap();
        if bytes_read == 0 {
            break;
        }
        stdout.write_all(buffer.as_bytes()).await.unwrap();
        stdout.flush().await.unwrap();
    }
}
