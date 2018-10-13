#[macro_export]
macro_rules! app {
    ($state:ident) => {
        App::with_state($state)
    }
}

#[macro_export]
macro_rules! middleware {
    ($app:ident, $middleware:tt) => {
        let $app = $app.middleware($middleware::default());
    }
}

#[macro_export]
macro_rules! route {
    ($app:ident, $route:tt, $handler:tt) => {
        let $app = $app.resource($route, |r| r.route().f($handler));
    }
}