use anyhow::Result;
use bytes::BytesMut;
use prost::Message;
use tokio_util::codec::{Framed, LengthDelimitedCodec};
use futures::prelude::*;
use common::proto::{MessageRequest, MessageResponse, message_response::Inner};
use tokio::net::TcpStream;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let addr = "127.0.0.1:7878";
    // 连接服务器
    let stream = TcpStream::connect(addr).await?;

    let mut client = Framed::new(stream, LengthDelimitedCodec::new());

    // 生成一个 HSET 命令
    let msg = MessageRequest::new_connect("9527".to_string(), 18);

    let mut buffer = BytesMut::new();

    msg.encode(&mut buffer).unwrap();

    // 发送 HSET 命令
    client.send(buffer.freeze()).await?;

    client.flush().await?;

    // thread::sleep(Duration::from_secs(2));

    // 生成一个 HSET 命令
    let msg = MessageRequest::new_text(20000, 9527, 9528, 0, "Hello koofox");

    let mut buffer = BytesMut::new();

    msg.encode(&mut buffer).unwrap();

    // 发送 HSET 命令
    client.send(buffer.freeze()).await?;


    loop {
        if let Some(Ok(data)) = client.next().await {

            let resp = MessageResponse::decode(&data[..]).unwrap();
    
            info!("Got response {:?}", resp);

            if let Some(Inner::Disconnect(_)) = resp.inner {
                break;
            }
        }
    }

    Ok(())
}
