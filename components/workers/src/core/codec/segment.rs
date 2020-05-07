use crate::protos::storage::{SegmentId, SegmentResolution};



pub fn encode_id(segment_id: SegmentId) -> String {
    let resolution = match segment_id.get_resolution() {
        SegmentResolution::DAY => "DAY",
        SegmentResolution::HOUR => "HOUR",
    };
    resolution.to_string()  + "#" + segment_id.get_timestamp()+ "#" + segment_id.get_id()
}

pub fn decode_id(segment_id_string: String) -> SegmentId {
    let parts: Vec<String> = segment_id_string.split('#').collect();
    let segment_id = SegmentId::new()
    segment_id.set_id(parts[0]);
    segment_id.set_id(parts[1]);
    segment_id.set_id(parts[2]);
   segment_id 
}

mod tests {
    use super::*;

    #[test]
    fn test_encode_decode() {
        let mut segment_id = SegmentId::default();
        segment_id.set_id("123".to_string());
        segment_id.set_timestamp("123-123-123".to_string());
        segment_id.set_resolution(SegmentResolution::DAY);
        let encoded_string = encode_id(segment_id);
        let decoded_segment_id = decode_id(encoded_string);

        assert!(segment_id == decoded_segment_id);

    }

}