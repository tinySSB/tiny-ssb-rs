use crate::feed_id::FeedId;
use crate::go_set::{GOSet, GOSetXor};

use crate::Error;

#[derive(Debug)]
pub struct GOSetClaim {
    lowest_feed_id: FeedId,
    highest_feed_id: FeedId,
    xor: GOSetXor,
    count: u8,
}

impl GOSetClaim {
    const GOSET_DMX: [u8; 7] = [0, 1, 2, 3, 4, 5, 6];
    // TODO: calculate actual DMX
    // ```
    // GOSET_DMX_MATERIAL = "tinySSB-0.1 GOset 1"
    // GOSET_DMX = first 7 bytes of SHA256(GOSET_DMX_MATERIAL)
    // ```

    pub fn encode(go_set: &GOSet) -> [u8; 105] {
        // NOTE: we're gonna want to make a range of claims
        // 1. how do we deal with the concept of a "subset" of a claim?
        // 2. where does the logic live about comparing a claim with a goset?
        // 3. what's the algorithm for the claim-dance?

        // [DMX (7B) | 'c' (1 byte) | lowest FeedId (32 bytes) | highest FeedId (32 bytes) | XOR (32 bytes) | cnt (1 byte) ]
        let mut claim = [0; 105];

        let mut offset: usize = 0;

        // DMX
        for (i, byte) in Self::GOSET_DMX.into_iter().enumerate() {
            claim[offset + i] = byte;
        }
        offset += Self::GOSET_DMX.len();

        // c
        let byte = "c".as_bytes()[0];
        claim[offset] = byte;
        offset += 1;

        // lowest FeedId
        let lowest_feed_id_bytes = go_set.lowest_feed_id().unwrap().encode();
        for (i, byte) in lowest_feed_id_bytes.into_iter().enumerate() {
            claim[offset + i] = byte;
        }
        offset += lowest_feed_id_bytes.len();

        // highest FeedId
        let highest_feed_id_bytes = go_set.highest_feed_id().unwrap().encode();
        for (i, byte) in highest_feed_id_bytes.into_iter().enumerate() {
            claim[offset + i] = byte;
        }
        offset += highest_feed_id_bytes.len();

        let xor = go_set.xor().encode();
        for (i, byte) in xor.into_iter().enumerate() {
            claim[offset + i] = byte;
        }
        offset += xor.len();

        let count = go_set.count();
        claim[offset] = count;

        claim
    }

    pub fn decode(bytes: &[u8; 105]) -> Result<Self, Error> {
        let mut offset: usize = 0;
        // TODO: check DMX
        offset += Self::GOSET_DMX.len();

        // TODO: check type: c
        offset += 1;

        let mut lowest_feed_id_bytes = [0; FeedId::LENGTH];
        for i in 0..FeedId::LENGTH {
            lowest_feed_id_bytes[i] = bytes[offset + i];
        }
        offset += FeedId::LENGTH;

        let mut highest_feed_id_bytes = [0; FeedId::LENGTH];
        for i in 0..FeedId::LENGTH {
            highest_feed_id_bytes[i] = bytes[offset + i];
        }
        offset += FeedId::LENGTH;

        let mut xor_bytes = [0; GOSetXor::LENGTH];
        for i in 0..GOSetXor::LENGTH {
            xor_bytes[i] = bytes[offset + i];
        }
        offset += GOSetXor::LENGTH;

        let count = bytes[offset];

        Ok(GOSetClaim {
            lowest_feed_id: FeedId(lowest_feed_id_bytes),
            highest_feed_id: FeedId(highest_feed_id_bytes),
            xor: GOSetXor(xor_bytes),
            count: count,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn claim_round_trip() {
        let feed_a = FeedId([255; 32]);
        let feed_b = FeedId([1; 32]);
        let feed_c = FeedId([2; 32]);

        let go_set = GOSet::new(&[feed_a, feed_b, feed_c]);
        let claim = GOSetClaim::encode(&go_set);
        println!("encoded claim: {:?}", claim);

        let result = GOSetClaim::decode(&claim).unwrap();
        println!("decoded claim: {:?}", result);

        assert_eq!(&result.lowest_feed_id, go_set.lowest_feed_id().unwrap());
        assert_eq!(&result.highest_feed_id, go_set.highest_feed_id().unwrap());
        assert_eq!(result.xor, go_set.xor());
        assert_eq!(result.count, go_set.count());
    }
}
