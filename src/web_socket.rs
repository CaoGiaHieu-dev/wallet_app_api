use async_tungstenite::tungstenite::Message;
use rocket::futures::stream::{SplitSink, SplitStream};
use rocket::futures::{SinkExt, StreamExt};
use rocket::request::{FromRequest, Request};
use rocket::tokio::sync::Mutex;
use rocket::State;
use rocket::{get, response::Responder, routes, Rocket};
use std::io::Cursor;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::net::TcpStream;
use tokio::sync::broadcast::{channel, Sender};
use tokio::time::{sleep, Duration};
use tokio_tungstenite::accept_async_with_config;
use tokio_tungstenite::tungstenite::protocol::WebSocketConfig;
use tokio_tungstenite::WebSocketStream;
use tungstenite::Error;

pub struct WebSocketResponder(SplitSink<WebSocketStream<TcpStream>, Message>);

impl<'r> Responder<'r, 'static> for WebSocketResponder {
    fn respond_to(self, _: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        let mut ws_sender = self.0;

        let ws_response = async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(1));

            loop {
                interval.tick().await;

                let message = Message::Text("Hello from server!".into());
                if let Err(err) = ws_sender.send(message.clone()).await {
                    eprintln!("Failed to send WebSocket message: {:?}", err);
                    break;
                }
            }

            Ok::<(), Error>(())
        };

        rocket::tokio::task::spawn_blocking(move || {
            rocket::tokio::runtime::Handle::current().block_on(ws_response)
        });

        Ok(rocket::response::Response::build()
            .streamed_body(Cursor::new(Vec::new()))
            .finalize())
    }
}

#[get("/ws")]
pub async fn websocket_handler(sender: &State<Mutex<Sender<Message>>>) -> WebSocketResponder {
    let addr = "127.0.0.1:9011"; // Replace with the desired IP address and port
    let stream = TcpStream::connect(addr)
        .await
        .expect("Failed to establish TCP connection");

    let ws_config = Some(WebSocketConfig::default()); // Optional configuration
    let ws_stream = accept_async_with_config(stream, ws_config).await.unwrap();

    let (ws_sender, ws_receiver) = ws_stream.split();

    let _ = ws_receiver.map(|e| println!("ws_receiver {:?}", e));

    WebSocketResponder(ws_sender)
}
