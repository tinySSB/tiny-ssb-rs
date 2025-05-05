mod go_set_claim;
mod go_set_xor;

use crate::feed_id::FeedId;

pub use go_set_claim::GOSetClaim;
pub use go_set_xor::GOSetXor;

#[derive(Debug)]
pub enum Error {
    TooManyFeedIds,
}

pub struct GOSet {
    feed_ids: Vec<FeedId>,
}
impl GOSet {
    // PG: I made this return a result because it's better to ensure the GoSet is valid at
    // construction time than blow up later on. If you've got a GoSet instance it's valid.
    pub fn new(feed_ids: &[FeedId]) -> Result<Self, Error> {
        if feed_ids.len() > u8::MAX as usize {
            return Err(Error::TooManyFeedIds);
        }
        let mut new_feed_ids = Vec::from(feed_ids);
        // TODO: discuss from() magic with Piet
        new_feed_ids.sort();

        Ok(Self {
            feed_ids: new_feed_ids,
        })
    }

    pub fn count(&self) -> u8 {
        // NOTE: u8 is the max currently supported by the spec
        self.feed_ids.len().try_into().unwrap()
    }

    pub fn xor(&self) -> GOSetXor {
        let mut xor: [u8; 32] = [0; 32];
        for feed_id in self.feed_ids.iter() {
            for i in 0..32 {
                xor[i] ^= feed_id.0[i];
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

    pub fn create_claim(&self) -> Option<GOSetClaim> {
        match (
            self.lowest_feed_id().cloned(),
            self.highest_feed_id().cloned(),
        ) {
            (Some(lowest_feed_id), Some(highest_feed_id)) => {
                let xor = self.xor();
                let count = self.count();

                Some(GOSetClaim {
                    highest_feed_id,
                    lowest_feed_id,
                    xor,
                    count,
                })
            }
            _ => None,
        }
    }
}

impl PartialEq<GOSetClaim> for GOSet {
    fn eq(&self, other: &GOSetClaim) -> bool {
        // TODO: If we change the GoSetClaim feeds to be options then we'll need to update the
        // comparison here.
        self.xor() == other.xor
            && self.highest_feed_id() == Some(&other.highest_feed_id)
            && self.lowest_feed_id() == Some(&other.lowest_feed_id)
            && self.count() == other.count
    }
}

impl PartialEq<GOSet> for GOSetClaim {
    fn eq(&self, other: &GOSet) -> bool {
        // TODO: If we change the GoSetClaim feeds to be options then we'll need to update the
        // comparison here.
        self.xor == other.xor()
            && Some(&self.highest_feed_id) == other.highest_feed_id()
            && Some(&self.lowest_feed_id) == other.lowest_feed_id()
            && self.count == other.count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count() {
        let feed_a = FeedId([255; 32]);
        let feed_b = FeedId([1; 32]);

        let go_set = GOSet::new(&[feed_a, feed_b]).unwrap();
        assert_eq!(go_set.count(), 2)
    }

    #[test]
    fn xor() {
        let feed_a = FeedId([255; 32]);
        let feed_b = FeedId([1; 32]);

        let go_set = GOSet::new(&[feed_a, feed_b]).unwrap();
        assert_eq!(go_set.xor().encode(), [254; 32]);
    }

    #[test]
    fn highest_feed_id() {
        let feed_a = FeedId([1; 32]);
        let feed_b = FeedId([2; 32]);
        let feed_c = FeedId([3; 32]);

        let go_set = GOSet::new(&[feed_b, feed_a, feed_c]).unwrap();

        assert_eq!(go_set.highest_feed_id().unwrap(), &feed_c);
    }

    #[test]
    fn lowest_feed_id() {
        let feed_a = FeedId([1; 32]);
        let feed_b = FeedId([2; 32]);
        let feed_c = FeedId([3; 32]);

        let go_set = GOSet::new(&[feed_b, feed_a, feed_c]).unwrap();

        assert_eq!(go_set.lowest_feed_id().unwrap(), &feed_a);
    }
}
