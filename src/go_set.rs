use crate::feed_id::FeedId;
use crate::go_set_xor::GOSetXor;

pub struct GOSet {
    feed_ids: Vec<FeedId>,
}
impl GOSet {
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

