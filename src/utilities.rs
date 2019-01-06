use std::time::{SystemTime, UNIX_EPOCH};


/// timestamp snowflake ID generator
/// TODO: use database instead of app server
pub fn get_millis() -> u64 {
  let start = SystemTime::now();
  let since_epoch = start.duration_since(UNIX_EPOCH)
    .expect("Time went backwards");
  since_epoch.as_secs() * 1000 +
    since_epoch.subsec_nanos() as u64 / 1_000_000 << 22
}
