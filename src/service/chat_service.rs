// use crate::models::chat_message_model::ChatMessageModel;
// use crate::repositories::mongo_repository::MongoRepo;
// use lazy_static::lazy_static;
// use std::sync::Mutex;

// lazy_static! {
//     static ref CURRENT_ROOM: Mutex<Option<i64>> = {
//         let res: Option<i64> = Some(1);
//         Mutex::new(res)
//     };
// }
// pub struct ChatService<'a>(&'a State<MongoRepo>);

// impl<'a> ChatService<'a> {
//     pub fn join_room(room_id: i64) {
//         let _ = CURRENT_ROOM.lock().unwrap().insert(room_id);

//         println!("join_room");

//         println!("{:?}", CURRENT_ROOM.lock().ok().unwrap());
//     }

//     pub fn receive_in_room_message(data: &ChatMessageModel) -> Result<&ChatMessageModel, String> {
//         println!(
//             "{:?}",
//             CURRENT_ROOM.lock().ok().unwrap().to_owned().is_none()
//         );
//         println!("{:?}", CURRENT_ROOM.lock().ok().unwrap());

//         if CURRENT_ROOM.lock().ok().unwrap().to_owned().is_none() {
//             return Err("Not join room yet".to_owned());
//         }

//         match CURRENT_ROOM.lock().ok().unwrap().unwrap().to_owned() == data.room {
//             true => return Ok(data),
//             false => return Err("Not join room yet".to_owned()),
//         }
//     }

//     pub fn leave_room() {
//         // CURRENT_ROOM.lock().ok().unwrap().as_ref() = None
//     }
// }
