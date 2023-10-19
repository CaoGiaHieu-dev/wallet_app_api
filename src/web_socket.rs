// use rocket::futures::{SinkExt, StreamExt};

// use serde_json::Value;
// use std::borrow::Cow;

// use ws::frame::{CloseCode, CloseFrame};
// use ws::Message;

// use serde::{Deserialize, Serialize};

// use crate::models::chat_message_model::ChatMessageModel;

// use crate::service::chat_service::ChatService;
// use crate::utils::constants::{JOIN_ROOM, SENDER_MESSAGE};
// use crate::utils::helper;

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct SocketEmitEvent {
//     pub event: String,
//     pub data: Option<Value>,
// }

// #[get("/ws", rank = 2)]
// pub fn echo_channel(ws: ws::WebSocket) -> ws::Channel<'static> {
//     let ws = ws.config(ws::Config::default());

//     ws.channel(move |mut stream| {
//         Box::pin(async move {
//             while let Some(message) = stream.next().await {
//                 if message.is_ok() {
//                     let message_ok = message.ok().clone();
//                     if message_ok.is_some() {
//                         let receive_message = message_ok.unwrap().to_text().unwrap().to_string();

//                         let decode_message = helper::decode_json(&receive_message);
//                         if decode_message.is_err() {
//                             let close = Message::Close(Some(CloseFrame {
//                                 code: CloseCode::Error,
//                                 reason: Cow::Owned(String::new()),
//                             }));
//                             let _ = stream.send(close).await;
//                         }

//                         let message_event = SocketEmitEvent::deserialize(decode_message.unwrap());

//                         if message_event.is_err() {
//                             let close = Message::Close(Some(CloseFrame {
//                                 code: CloseCode::Error,
//                                 reason: Cow::Owned(String::new()),
//                             }));
//                             let _ = stream.send(close).await;
//                         }

//                         let socket_emit_event = message_event.unwrap();

//                         let mut message_response = Message::Close(None);

//                         print!("{:?}", socket_emit_event.data);

//                         if socket_emit_event.event.to_uppercase() == SENDER_MESSAGE {
//                             let message_sender =
//                                 ChatMessageModel::deserialize(socket_emit_event.data.unwrap())
//                                     .expect("Cannot parse message");

//                             let checker = ChatService::receive_in_room_message(&message_sender);

//                             print!("{:?}", checker);

//                             if checker.is_ok() {
//                                 let _checker = checker.unwrap().clone();
//                                 message_response = Message::Text(_checker.message);
//                             } else {
//                                 message_response = Message::Text(checker.err().unwrap());
//                             }
//                         } else if socket_emit_event.event.to_uppercase() == JOIN_ROOM {
//                             let room_id = socket_emit_event
//                                 .data
//                                 .expect("Cannot match")
//                                 .as_i64()
//                                 .expect("Not match type");

//                             ChatService::join_room(room_id);

//                             message_response = Message::Text("Ok".to_owned());
//                         }

//                         let sender = stream.send(message_response).await;

//                         if sender.is_err() {
//                             let close = Message::Close(Some(CloseFrame {
//                                 code: CloseCode::Error,
//                                 reason: Cow::Owned(String::new()),
//                             }));
//                             let _ = stream.send(close).await;
//                         }
//                     }
//                 }
//             }

//             Ok(())
//         })
//     })
// }
