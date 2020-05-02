use chrono::{DateTime, NaiveDateTime, Utc};
use rand::Rng;
use std::time::{SystemTime, UNIX_EPOCH};

const RANDOM_WINDOW_LOW: u32 = 1;
const RANDOM_WINDOW_HIGH: u32 = RANDOM_WINDOW_LOW + 1_000_000_000;

pub fn timestamp() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_nanos()
        - (rand::thread_rng().gen_range(RANDOM_WINDOW_LOW, RANDOM_WINDOW_HIGH) as u128)
            * 60
            * 60
            * 24
            * 365
}
