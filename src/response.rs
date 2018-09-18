use actix_web;
use serde;
use serde_json;


// /// Extend actix_web Json helper with Respondable trait
// trait Respondable {
//     fn response<S: 'static>(
//         self, req: &HttpRequest<S>, status: http::StatusCode,
//     ) -> Result<HttpResponse, Error>;
// }
// impl<T: Serialize> Respondable for Json<T> {
//     fn response<S>(self, req: &HttpRequest<S>, status: http::StatusCode) -> Result<HttpResponse, Error> {
//         let body = serde_json::to_string(&self.0)?;

//         return Ok(req
//             .build_response(status)
//             .content_type("application/json")
//             .body(body));
//     }
// }

pub fn json<C, S>(req: &actix_web::HttpRequest<C>,
                      object: S,
                      status: actix_web::http::StatusCode)
    -> Result<actix_web::HttpResponse, actix_web::Error>
    where S: serde::Serialize
{
    let body = serde_json::to_string(&object)?;

    return Ok(req
        .build_response(status)
        .content_type("application/json")
        .body(body));
}
