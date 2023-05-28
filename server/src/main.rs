use std::{collections::HashMap, sync::Arc, net::Shutdown};

use bytes::BytesMut;
use prost::Message;
use std::net::{TcpListener, TcpStream};
use tokio::sync::{Mutex, mpsc};
use anyhow::Result;
use tracing::info;
use futures::prelude::*;

use common::proto::{MessageRequest, message_request};
use tokio_util::codec::{Framed, LengthDelimitedCodec};

use crate::service::{Service, ServiceInner};

mod service;

#[allow(unused_variables)]
#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let addr = "127.0.0.1:7878";
    let listener = TcpListener::bind(addr).unwrap();
    info!("Start listening on {}", addr);

    let clients = Arc::new(Mutex::new(HashMap::<String, TcpStream>::new()));


    let (sender, receiver) = mpsc::unbounded_channel::<Box<dyn Message>>();

    let service: Service = ServiceInner::new().into();

    loop {
        let (raw_stream, addr) = listener.accept().unwrap();
        info!("Client: {:?} connected", addr);

        let addr = addr.to_string();

        let mut data = clients.lock().await;
        data.insert(addr.clone(), raw_stream.try_clone().unwrap());

        let service = service.clone();

        let clients = Arc::clone(&clients);

        tokio::spawn(async move {
            let tokio_stream = tokio::net::TcpStream::from_std(raw_stream.try_clone().unwrap()).unwrap();
            let mut stream = Framed::new(tokio_stream, LengthDelimitedCodec::new());
            while let Some(Ok(mut buf)) = stream.next().await {

                let msg = MessageRequest::decode(&buf[..]).unwrap();
                println!("recevie: {:?}", msg);

                buf.clear();

                match msg.clone().inner {
                    Some(message_request::Inner::Connect(req)) => {
                        let mut data = clients.lock().await;

                        let target_stream = match data.remove(&addr) {
                            Some(stream) => stream,
                            None => raw_stream.try_clone().unwrap()
                        };

                        if let Some(old_stream) = data.insert(req.identifier, target_stream) {
                            let _ = old_stream.shutdown(Shutdown::Both);
                        }

                        if let Some(res) = service.execute(msg) {
                            println!("res1:{:?}", res);
                            let mut buf = BytesMut::new();
                            res.encode(&mut buf).unwrap();
                            stream.send(buf.freeze()).await.unwrap();
                        }
                    },
                    Some(message_request::Inner::Text(req)) => {
                        println!("test:{:?}", req);

                        let mut data = clients.lock().await;

                        if let Some(stream) = data.remove(&req.receiver_id.to_string()) {
                            println!("test1:{:?}", &req.receiver_id.to_string());

                            let tokio_stream = tokio::net::TcpStream::from_std(stream).unwrap();
                            let mut stream = Framed::new(tokio_stream, LengthDelimitedCodec::new());
                            
                            let res = MessageRequest::new_text(req.message_id, req.sender_id, req.receiver_id, req.message_type, &req.content);
                            let mut buf = BytesMut::new();
                            res.encode(&mut buf).unwrap();
                            stream.send(buf.freeze()).await.unwrap();
                        }

                        if let Some(res) = service.execute(msg) {
                            println!("res2:{:?}", res);
                            let mut buf = BytesMut::new();
                            res.encode(&mut buf).unwrap();
                            stream.send(buf.freeze()).await.unwrap();
                        }
                    }
                    _ => (),
                }
                
            }
        });
    }
}

