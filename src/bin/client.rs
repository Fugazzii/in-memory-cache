/**
 * This is just an example how client can interact with in-memory-cache server
 * It creates TCP connection with the server
 */

use bytes::BytesMut;
use clap::Parser;
use tokio::{
    net::TcpStream, 
    io::AsyncWriteExt
};
use tokio::io::AsyncReadExt;

use in_memory_cache::cli;
use cli::{Cli, ClientCommand};

#[tokio::main]
pub async fn main() -> Result<(), std::io::Error> {
    let args = Cli::parse();

    let mut stream = TcpStream::connect("127.0.0.1:3000")
        .await
        .expect("Failed to connect to serve");
    
    match args.command {
        ClientCommand::Set { key, value } => {

            /* Header buffer  */
            stream.write_all(b"set").await.expect("Could not send buffer");
            stream.write_all(b" ").await.expect("Could not send buffer");
            stream.write_all(&key.as_bytes()).await.expect("Could not send buffer");
            stream.write_all(b" ").await.expect("Could not send buffer");
            stream.write_all(&value.as_bytes()).await.expect("Could not send buffer");

            let mut buf = BytesMut::with_capacity(1024);
            let _length = stream.read_buf(&mut buf).await?;

            match std::str::from_utf8(&mut buf) {
                Ok(resp) => {
                    if resp == "r Ok" {
                        println!("updated key");
                    } else if resp == "Ok" {
                        println!("key set");
                    }
                }
                 Err(err) => {
                    // failed to convert bytes into string slice
                    println!("error: {}", err);
                }
            }
            Ok(())
        }
        ClientCommand::Get { key } => {
            /* Header buffer */
            stream.write_all(b"get").await?;
            stream.write_all(b" ").await?;
            stream.write_all(&key.as_bytes()).await?;

            let mut buf = BytesMut::with_capacity(1024);
            let _length = stream.read_buf(&mut buf).await?;
            match std::str::from_utf8(&mut buf) {
                Ok(resp) => {
                    if resp == "" {
                        println!("no such key found");
                    } else {
                        println!("key: {} => value: {}", key, resp);
                    }
                }
                Err(_err) => {
                    println!("in errr");
                }
            }
            return Ok(());
        }
    }

}