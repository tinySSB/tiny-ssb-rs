mod go_set_claim;
mod go_set_xor;

use crate::feed_id::FeedId;

pub use go_set_claim::GOSetClaim;
pub use go_set_xor::GOSetXor;

pub struct GOSet {
    feed_ids: Vec<FeedId>,
}
impl GOSet {
    pub fn new(feed_ids: &[FeedId]) -> Self {
        let mut new_feed_ids = Vec::from(feed_ids);
        // TODO: discuss from() magic with Piet
        new_feed_ids.sort();

        Self {
            feed_ids: new_feed_ids,
        }
    }
    pub fn count(&self) -> u8 {
        todo!()
    }
    pub fn xor(&self) -> GOSetXor {
        let mut xor: [u8; 32] = [0; 32];
        for feed_id in self.feed_ids.iter() {
            for i in 0..32 {
                xor[i] = xor[i] ^ feed_id.0[i];
            }
        }
        GOSetXor(xor)
    }
    pub fn highest_feed_id(&self) -> Option<&FeedId> {
        self.feed_ids.last()
    }
    pub fn lowest_feed_id(&self) -> Option<&FeedId> {
        self.feed_ids.first()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn xor() {
        let feed_a = FeedId([255; 32]);
        let feed_b = FeedId([1; 32]);

        let go_set = GOSet::new(&[feed_a, feed_b]);
        assert_eq!(go_set.xor().encode(), [254; 32]);
    }

    #[test]
    fn highest_feed_id() {
        let feed_a = FeedId([1; 32]);
        let feed_b = FeedId([2; 32]);
        let feed_c = FeedId([3; 32]);

        let go_set = GOSet::new(&[feed_b, feed_a, feed_c]);

        assert_eq!(go_set.highest_feed_id().unwrap(), &feed_c);
    }

    #[test]
    fn lowest_feed_id() {
        let feed_a = FeedId([1; 32]);
        let feed_b = FeedId([2; 32]);
        let feed_c = FeedId([3; 32]);

        let go_set = GOSet::new(&[feed_b, feed_a, feed_c]);

        assert_eq!(go_set.lowest_feed_id().unwrap(), &feed_a);
    }
}
