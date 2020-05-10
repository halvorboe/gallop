use crate::protos::common::Row;
use crate::protos::common::SegmentResolution;

use chrono::{DateTime, NaiveDateTime, Utc};

pub fn encode(row: &Row) -> String {
    row.get_timestamp().to_string() + "#" + &row.get_data().to_string()
}

pub fn decode(row_string: &String) -> Row {
    let parts: Vec<String> = row_string.split('#').map(|it| it.to_string()).collect();
    let mut row = Row::new();
    row.set_timestamp(parts[0].parse().unwrap());
    row.set_data(parts[1].clone());
    row
}

pub fn encode_timestamp(timestamp: u64, resolution: SegmentResolution) -> String {
    let naive = NaiveDateTime::from_timestamp(
        (timestamp / 1_000_000_000) as i64,
        timestamp as u32 % 1_000_000_000,
    );
    let datetime: DateTime<Utc> = DateTime::from_utc(naive, Utc);
    let formatter = match resolution {
        // SegmentResolution::HOUR => "%Y#%m#%d#%H",
        SegmentResolution::DAY => "%Y-%m-%d",
    };
    datetime.format(formatter).to_string()
}
