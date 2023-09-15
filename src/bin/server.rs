use in_memory_cache::{buffer_to_array, Command, Db};
use tokio::{net::{TcpListener, TcpStream}, io::AsyncWriteExt};
use bytes::BytesMut;
use tokio::io::AsyncReadExt;

#[tokio::main]
pub async fn main() -> Result<(), std::io::Error> {

    // Listen incoming requests
    let listener = TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("Error while listening");

    // Actual data holder structure
    let mut db = Db::new();

    loop {
        let (mut socket, _) = listener
            .accept()
            .await
            .expect("Error in socket");

        println!("Connection accepted");

        // Get buffer
        let mut buf = BytesMut::with_capacity(1024);

        // Wait for incoming buffer from client
        let _length = socket.read_buf(&mut buf).await?;

        // Get full input from user        
        let attrs = buffer_to_array(&mut buf);

        // Retreive command
        let command = Command::get(&attrs[0]);        
        
        process_query(command, attrs, &mut socket, &mut db).await.expect("Failed to process query");

        println!("{:?}", buf);    
    }

    // Ok(())
}

async fn process_query(
    command: Command,
    attrs: Vec<String>,
    socket: &mut TcpStream,
    db: &mut Db
) -> std::io::Result<()> {
    match command {
        Command::Get => {
            let res = db.read(&attrs);
            match res {
                Ok(res) => {
                    socket.write_all(&res).await.expect("Failed to retreive value from database");
                }
                Err(err) => {
                    println!("no key found {:?}", err);
                    socket.write_all(b"").await?;
                }
            }
            Ok(())
        }
        Command::Set => {
            let res = db.write(&attrs);

            match res {
                Ok(res) => {
                    println!("New item: {}", res);

                    socket.write_all(&res.as_bytes()).await.expect("Failed to write return response after setting value");
                }
                Err(_err) => {
                    socket.write_all(b"").await?;
                }
            }
            Ok(())
        }
        Command::Invalid => {
            Ok(())
        }
    }
}