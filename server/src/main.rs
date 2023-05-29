use std::{collections::HashMap, sync::Arc};

use bytes::BytesMut;
use prost::Message;
use std::net::{TcpListener};
use tokio::sync::{Mutex, mpsc, broadcast};
use anyhow::Result;
use tracing::info;
use futures::prelude::*;

use common::proto::{MessageRequest, message_request, MessageResponse};
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

    let clients = Arc::new(Mutex::new(HashMap::<String, mpsc::UnboundedSender<MessageResponse>>::new()));

    let service: Service = ServiceInner::new().into();

    let (tx, _) = broadcast::channel::<MessageResponse>(2);

    loop {
        let (stream, addr) = listener.accept().unwrap();
        info!("Client: {:?} connected", addr);

        let service = service.clone();

        let clients = Arc::clone(&clients);

        let tx = tx.clone();
        let mut rx = tx.subscribe();


        tokio::spawn(async move {
            let stream = tokio::net::TcpStream::from_std(stream.try_clone().unwrap()).unwrap();
            let mut stream = Framed::new(stream, LengthDelimitedCodec::new());

            let (sender, mut receiver) = mpsc::unbounded_channel::<MessageResponse>();

            loop {
                tokio::select! {
                    Some(msg) = receiver.recv() => {
                        let mut buf = BytesMut::new();
                        msg.encode(&mut buf).unwrap();
                        stream.send(buf.freeze()).await.unwrap();
                    },
                    Ok(msg) = rx.recv() => {
                        println!("broastcast recv");
                        let mut buf = BytesMut::new();
                        msg.encode(&mut buf).unwrap();
                        stream.send(buf.freeze()).await.unwrap();
                    },
                    Some(Ok(mut buf)) = stream.next() => {

                        let msg = MessageRequest::decode(&buf[..]).unwrap();
                        println!("recevie: {:?}", msg);
        
                        buf.clear();
        
                        match msg.clone().inner {
                            Some(message_request::Inner::Connect(req)) => {
        
                                let mut data = clients.lock().await;
                                if let Some(old_sender) = data.insert(req.identifier, sender.clone()) {
                                    old_sender.closed().await;
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
        
                                if let Some(sender) = data.get_mut(&req.receiver_id.to_string()) {
                                    sender.send(MessageResponse::with_text(req.clone())).unwrap();
                                }
        
                                if req.message_type == 1 {
                                    tx.send(MessageResponse::with_text(req.clone())).unwrap();
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
                        
                    },
                    else => break,
                }
            }
        });
    }
}

