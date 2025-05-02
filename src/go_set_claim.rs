use crate::feed_id::FeedId;
use crate::go_set::GOSet;
use crate::go_set_xor::GOSetXor;

use crate::Error;

pub struct GOSetClaim {
    lowest_feed_id: FeedId,
    highest_feed_id: FeedId,
    xor: GOSetXor,
    count: u8,
}

impl GOSetClaim {
    pub fn encode_go_set(go_set: &GOSet) -> [u8; 105] {
        todo!()
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, Error> {
        todo!()
    }
}
