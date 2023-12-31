// use rocket::form::Form;

// use rocket::response::stream::{Event, EventStream};
// use rocket::serde::{Deserialize, Serialize};
// use rocket::tokio::select;
// use rocket::tokio::sync::broadcast::{error::RecvError, Sender};
// use rocket::{Shutdown, State};

// #[derive(Debug, Clone, FromForm, Serialize, Deserialize)]
// #[cfg_attr(test, derive(PartialEq, UriDisplayQuery))]
// #[serde(crate = "rocket::serde")]
// pub struct Message {
//     #[field(validate = len(..30))]
//     pub room: String,
//     #[field(validate = len(..20))]
//     pub username: String,
//     pub message: String,
// }

// #[get("/events")]
// pub fn events(queue: &State<Sender<Message>>, mut end: Shutdown) -> Result<EventStream![], ()> {
//     let mut rx = queue.subscribe();
//     Ok(EventStream! {
//         loop {
//             let msg = select! {

//                 msg = rx.recv() => match msg {
//                     Ok(msg) => msg,
//                     Err(RecvError::Closed) => break,
//                     Err(RecvError::Lagged(_)) => continue,
//                 },
//                 _ = &mut end => break,
//             };

//             yield Event::json(&msg);
//         }
//     })
// }

// /// Receive a message from a form submission and broadcast it to any receivers.
// #[post("/message", data = "<form>")]
// pub fn post(form: Form<Message>, queue: &State<Sender<Message>>) {
//     // A send 'fails' if there are no active subscribers. That's okay.
//     let _res = queue.send(form.into_inner());
// }
