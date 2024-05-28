use std::time::{SystemTime};

pub fn unix_second() -> u64{
    let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
    return now;
}
