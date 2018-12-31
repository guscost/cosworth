#[macro_export]
macro_rules! postgres {
  ($url:ident) => {
    diesel::r2d2::Pool::builder().build(
      diesel::r2d2::ConnectionManager::<diesel::prelude::PgConnection>::new($url)
    ).expect("Failed to create pool.")
  }
}
