pub enum Error {}

struct FeedId([u8; 32]);
impl FeedId {
    pub fn encode(&self) -> [u8; 32] {
        self.0.clone()
    }
}

struct GoSet {
    feed_ids: Vec<FeedId>,
}
impl GoSet {
    pub fn new(feed_ids: &[FeedId]) -> Self {
        todo!()
    }
    pub fn count(&self) -> u8 {
        todo!()
    }
    pub fn xor(&self) -> GOSetXor {
        todo!();
    }
    pub fn highest_feed_id(&self) -> FeedId {
        todo!()
    }
    pub fn lowest_feed_id(&self) -> FeedId {
        todo!()
    }
}
struct GOSetXor([u8; 32]);
impl GOSetXor {
    pub fn encode() -> [u8; 32] {
        todo!()
    }
}
struct GOSetClaim {
    lowest_feed_id: FeedId,
    highest_feed_id: FeedId,
    xor: GOSetXor,
    count: u8,
}

impl GOSetClaim {
    pub fn encode_go_set(go_set: &GoSet) -> [u8; 105] {
        todo!()
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, Error> {
        todo!()
    }
}

enum WirePacket {
    // Dunno about this. Might be bytes not sure
    Replication(GOSetClaim),
    Log(),
}

#[cfg(test)]
mod tests {
    use super::*;
    use bipf_rs::bipf::{Bipf, decode};

    #[test]
    fn round_trip() {
        let value = serde_json::Value::String("Mix is cool".to_string());

        let value_bytes = value.to_bipf();
        println!("{:?}", value_bytes);

        let decoded = decode(&value_bytes).unwrap();

        assert_eq!(value, decoded);
    }
}
