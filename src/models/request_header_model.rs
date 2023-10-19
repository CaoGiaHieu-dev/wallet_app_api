// use crate::utils::ErrorResponse;

// #[derive(Debug, Clone, Copy)]
// pub struct RequestHeaders<'h>(&'h HeaderMap<'h>);

// impl<'h> RequestHeaders<'h> {
//     pub fn get_one(&self, name: &str) -> Option<&'h str> {
//         self.0.get_one(name)
//     }
// }

// #[rocket::async_trait]
// impl<'r> FromRequest<'r> for RequestHeaders<'r> {
//     type Error = ErrorResponse;

//     async fn from_request(request: &'r Request<'_>) -> Outcome<Self, ErrorResponse> {
//         let request_headers = request.headers();

//         Outcome::Success(RequestHeaders(request_headers))
//     }
// }
