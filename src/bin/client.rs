use bytes::BytesMut;
use tokio::{net::TcpStream, io::{AsyncReadExt, AsyncWriteExt}};

#[tokio::main]
pub async fn main() -> Result<(), std::io::Error> {
    let mut stream = TcpStream::connect("127.0.0.1:3000")
        .await
        .expect("Failed to connect to server");
    
    stream.write_all(b"set foo bar").await?;

    let mut buf = BytesMut::with_capacity(1024);
    let _length = stream.read_buf(&mut buf).await?;
    match std::str::from_utf8(&mut buf) {
        Ok(resp) => {
            if resp == "r Ok" {
                println!("key updated");
            } else if resp == "Ok" {
                println!("key set");
            }
        }
        Err(err) => {
            println!("Failed to convert bytes into string slice: {}", err);
        }
    }

    Ok(())
}