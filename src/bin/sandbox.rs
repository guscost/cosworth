extern crate actix_web;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde;

#[macro_use]
extern crate cosworth;

use actix_web::{http, pred, server, App, Error, HttpRequest, HttpResponse};
use cosworth::response::json;


#[derive(Serialize, Deserialize, Debug)]
struct Widget {
    id: String,
    name: String,
}


fn index(req: &HttpRequest) -> Result<HttpResponse, Error> {
    let id = req.match_info().query("id")?;
    let name = req.match_info().query("name")?;

    if id == "123" {
        return Ok(req.build_response(http::StatusCode::PARTIAL_CONTENT)
              .content_type("text/plain")
              .body(hello!()));
    } else {
        let widget = Widget { id: id, name: name };
        return Ok(json(&req, widget, http::StatusCode::OK)?);
    }
}

fn main() {
    println!("{}", hello!());
    server::new(|| {
        App::new()
            .resource("/{id}/{name}", |r| {
                r.route()
                 .filter(pred::Get())
                 .filter(pred::Header("content-type", "application/json"))
                 .f(index)
            })
    })
        .bind("0.0.0.0:8080").unwrap()
        .run();
}
