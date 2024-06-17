use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

async fn process_stream(tcp_stream: TcpStream) -> Result<(), tokio::io::Error> {
    let (mut r, mut w) = tcp_stream.into_split();

    loop {
        let byte = r.read_u8().await?;

        w.write_u8(byte).await?;

        w.flush().await?;
    }

    #[allow(unreachable_code)]
    Ok(())
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:1337").await.unwrap();

    loop {
        match listener.accept().await {
            Ok((tcp_stream, remote_addr)) => {
                tracing::warn!(remote_addr=?remote_addr ,"accepted new socket");

                tokio::task::spawn({
                    async move {
                        if let Err(err) = process_stream(tcp_stream).await {
                            tracing::error!(cause=?err,remote_addr=?remote_addr, "failed to process stream")
                        }
                    }
                });
            }
            Err(err) => {
                tracing::error!(cause=?err, "failed to accept socket")
            }
        }
    }
}
