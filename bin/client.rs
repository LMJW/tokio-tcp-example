use tokio::net::{TcpStream};
use std::error::Error;
use tokio::io::AsyncWriteExt;

#[tokio::main]
async fn main()-> Result<(), Box<dyn Error>> {
    let mut stream = TcpStream::connect("0.0.0.0:8888").await?;

    stream.write_all(b"hello world").await?;
    stream.shutdown().await?;
    Ok(())
}