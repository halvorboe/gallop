#[derive(Debug, Clone)]
pub enum SegmentResolution {
    HOUR,
    DAY,
}

#[derive(Debug, Clone)]
pub struct Configuration {
    pub segment_resolution: SegmentResolution,
    pub segment_max_size: usize,
}

impl Configuration {
    pub fn default() -> Self {
        Self {
            segment_resolution: SegmentResolution::DAY,
            segment_max_size: 1_000_000,
        }
    }

    pub fn new(segment_resolution: SegmentResolution, segment_max_size: usize) -> Self {
        Self {
            segment_resolution,
            segment_max_size,
        }
    }
}
