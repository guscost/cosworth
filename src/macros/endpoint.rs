#[macro_export]
macro_rules! endpoint {
  ($type:ident, $name:ident) => {
    pub fn $name(req: &HttpRequest<AppState>) -> Box<Future<Item = HttpResponse, Error = Error>> {
      let req = req.clone();
      return req.body()
        .from_err()
        .and_then(move |body| {
          let mut path_params = HashMap::new();
          for (k, v) in req.match_info().iter() { path_params.insert(k.to_owned(), v.to_owned()); }
          let process_request = ProcessRequest {
            endpoint: &$type{},
            request: Request {
              method: req.method().to_string(),
              path_params: path_params,
              query_params: req.query().to_owned(),
              headers: req.headers().to_owned(),
              body: body
            }
          };
          return req.state().processors
            .send(process_request)
            .from_err()
            .and_then(|res| match res {
              Ok(obj) => {
                let mut builder = HttpResponse::build(http::StatusCode::from_u16(obj.status).unwrap());
                for (k, v) in obj.headers.iter() { builder.header(k, v.to_owned()); }
                Ok(builder.content_type("application/json").body(obj.body))
              },
              Err(_) => Ok(HttpResponse::InternalServerError().into()),
            });
        })
        .responder();
    }
  };
}
