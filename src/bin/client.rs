use tokio::{net::TcpStream, io::AsyncWriteExt};

#[tokio::main]
pub async fn main() -> Result<(), std::io::Error> {
    let mut stream = TcpStream::connect("127.0.0.1:3000")
        .await
        .expect("Failed to connect to server");

    stream.write_all(b"Jon Jones").await?;

    Ok(())
}