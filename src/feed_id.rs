use mem_cmp::*;
use std::cmp::Ordering;

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub struct FeedId(pub [u8; 32]);

impl FeedId {
    pub fn encode(&self) -> [u8; 32] {
        self.0.clone()
    }
}

impl PartialOrd for FeedId {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.0.mem_cmp(&other.0))
    }
}

impl Ord for FeedId {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.mem_cmp(&other.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // bring in everything from the module above, i.e. FeedId

    #[test]
    fn partial_eq() {
        // this might be testing eq??
        let feed_a = FeedId([0; 32]);
        let feed_b = FeedId([0; 32]);
        let feed_c = FeedId([4; 32]);

        assert_eq!(feed_a, feed_b);
        assert_ne!(feed_a, feed_c);
    }

    #[test]
    fn partial_ord() {
        let feed_a = FeedId([0; 32]);
        let feed_b = FeedId([4; 32]);

        assert!(feed_a.lt(&feed_b));
        assert!(feed_b.gt(&feed_a));

        assert!(!feed_a.lt(&feed_a));
    }
}
