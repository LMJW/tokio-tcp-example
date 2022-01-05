use tokio::net::{TcpListener};
use std::error::Error;
use std::sync::Arc;
#[tokio::main]
async fn main()-> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("0.0.0.0:8888").await?;

    loop {
        let (stream, addr) = listener.accept().await?;
        println!("receiving income connection");
        let stream = Arc::new(stream);
        let sc = stream.clone();
        tokio::spawn(async move{
            sc.readable().await.unwrap();

            let mut buf = vec![0;1000];
            loop{
                match sc.try_read(&mut buf){
                    Ok(n) => {
                        println!("received {} bytes form addr{}, {:?}", n, addr, &buf[..n]);
                        if n == 0{
                            println!("client closed the stream");
                            break;
                        }
                    }
                    Err(e) => {
                        println!("error: {}",e);
                    }
                };
            }
            println!("closing the async task");
        });

    }

    Ok(())
}