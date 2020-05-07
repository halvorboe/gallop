use crate::protos::packer::{SegmentId, SegmentResolution};



pub fn encode_id(segment_id: SegmentId) -> String {
    let resolution = match segment_id.get_resolution() {
        SegmentResolution::DAY => "DAY",
        SegmentResolution::HOUR => "HOUR",
    };
    segment_id.get_table().to_string() + "#" + resolution  + "#" + segment_id.get_timestamp()+ "#" + segment_id.get_id()
}

pub fn decode_id(segment_id_string: String) -> SegmentId {
    let parts: Vec<String> = segment_id_string.split('#').map(|it| it.to_string()).collect();
    let mut segment_id = SegmentId::new();
    segment_id.set_table(parts[0].clone());
    let day = String::from("DAY");
    let hour = String::from("HOUR");
    segment_id.set_resolution(match parts[1].clone() {
       day => SegmentResolution::DAY,
       hour => SegmentResolution::HOUR,
    });
    segment_id.set_timestamp(parts[2].clone());
    segment_id.set_id(parts[3].clone());
    segment_id 
}

mod tests {
    use super::*;

    #[test]
    fn test_encode_decode() {
        let mut segment_id = SegmentId::default();
        segment_id.set_table("a".to_string());
        segment_id.set_id("123".to_string());
        segment_id.set_timestamp("123-123-123".to_string());
        segment_id.set_resolution(SegmentResolution::DAY);
        let encoded_string = encode_id(segment_id.clone());
        let decoded_segment_id = decode_id(encoded_string);
        assert!(dbg!(segment_id) == dbg!(decoded_segment_id));

    }

}