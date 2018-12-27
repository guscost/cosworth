#[macro_export]
macro_rules! cosworth {
  (context!($db_pool:tt) $($tail:tt)*) => {
    // create actix system
    let sys = ActixSystem::new("cosworth-system");

    // read env variables
    let host = match std::env::var("COSWORTH_HOST") {
      Ok(v) => v,
      Err(_e) => "0.0.0.0:8080".to_owned()
    };
    let processors_num = match std::env::var("COSWORTH_PROCESSORS") {
      Ok(v) => v.parse::<usize>().expect("COSWORTH_PROCESSORS must be an integer."),
      Err(_e) => num_cpus()
    };

    // init processor actors
    println!("Starting {} request processors...", processors_num);
    let processors = ActixSyncArbiter::start(
      processors_num,
      move || Context{db: $db_pool.clone()}
    );

    // start actix server
    server::new(move || {
      let context = AppState{processors: processors.clone()};
      let app = App::with_state(context);
      cosworth!(app $($tail)*);
      return app;
    })
      .maxconnrate(4096)
      .bind(host.to_owned()).unwrap()
      .start();
    
    // run actix system
    println!("Server running at http://{}...", host);
    sys.run();
  };
  ($app:tt, middleware!($middleware:tt) $($tail:tt)*) => {
    let $app = $app.middleware($middleware::default());
    cosworth!($app $($tail)*);
  };
  ($app:tt, route!($route:tt, $endpoint:tt) $($tail:tt)*) => {
    let $app = $app.resource($route, |resource| {
      return resource.route().f(|req| -> Box<Future<Item = HttpResponse, Error = Error>> {
        let req = req.clone();
        return req.body()
          .from_err()
          .and_then(move |body| {
          let mut path_params = HashMap::new();
          for (k, v) in req.match_info().iter() { path_params.insert(k.to_owned(), v.to_owned()); }
          let process_request = RequestMessage {
            endpoint: &$endpoint{},
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
                let mut resp = HttpResponse::build(http::StatusCode::from_u16(obj.status).unwrap());
                for (k, v) in obj.headers.iter() { resp.header(k, v.to_owned()); }
                Ok(resp.content_type("application/json").body(obj.body))
              },
              Err(_) => Ok(HttpResponse::InternalServerError().into()),
            });
          })
          .responder();
      });
    });
    cosworth!($app $($tail)*);
  };
  ($app:tt $($tail:tt)*) => {};
}
