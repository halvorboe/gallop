use crate::protos::common::SegmentResolution;

#[derive(Debug, Clone)]
pub struct Configuration {
    pub segment_resolution: SegmentResolution,
    pub segment_max_size: u64,
}

impl Configuration {
    pub fn default() -> Self {
        Self {
            segment_resolution: SegmentResolution::DAY,
            segment_max_size: 1_000_000,
        }
    }

    pub fn new(segment_resolution: SegmentResolution, segment_max_size: u64) -> Self {
        Self {
            segment_resolution,
            segment_max_size,
        }
    }
}
