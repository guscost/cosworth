#[macro_export]
macro_rules! processors {
    ($num:tt, $db_pool:ident) => {{
        ActixSyncArbiter::start(3, move || Processor{db: $db_pool.clone()})
    }}
}

#[macro_export]
macro_rules! cosworth {
    ($app:tt, route!($route:tt, $handler:tt) $($tail:tt)*) => {
        let $app = $app.resource($route, |r| r.route().f($handler));
        cosworth!($app $($tail)*);
    };
    ($app:tt, middleware!($middleware:tt) $($tail:tt)*) => {
        let $app = $app.middleware($middleware::default());
        cosworth!($app $($tail)*);
    };
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
        let processors = processors!(processors_num, $db_pool);

        // start actix server
        server::new(move || {
            let context = Context{processors: processors.clone()};
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
    ($app:tt $($tail:tt)*) => {};
}
