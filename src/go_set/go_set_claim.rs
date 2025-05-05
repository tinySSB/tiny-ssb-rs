use crate::feed_id::FeedId;
use crate::go_set::{GOSet, GOSetXor};

use crate::Error;

#[derive(Debug, PartialEq)]
enum GOSetClaimError {
    InvalidDMX,
    InvalidTypeCode,
}

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

        let dmx = Self::GOSET_DMX;
        let type_code: [u8; 1] = [b'c'];
        let lowest_feed_id = go_set.lowest_feed_id().unwrap().encode();
        let highest_feed_id = go_set.highest_feed_id().unwrap().encode();
        let xor = go_set.xor().encode();
        let count: [u8; 1] = [go_set.count()];

        let chunks: [&[u8]; 6] = [
            &dmx,
            &type_code,
            &lowest_feed_id,
            &highest_feed_id,
            &xor,
            &count,
        ];

        let mut offset: usize = 0;
        for chunk in chunks {
            let len = chunk.len();
            claim[offset..offset + len].copy_from_slice(chunk);
            offset += len;
        }

        claim
    }

    pub fn decode(bytes: &[u8; 105]) -> Result<Self, GOSetClaimError> {
        let mut dmx = [0; Self::GOSET_DMX.len()];
        let mut type_code = [0; 1];
        let mut lowest_feed_id = [0; FeedId::LENGTH];
        let mut highest_feed_id = [0; FeedId::LENGTH];
        let mut xor = [0; GOSetXor::LENGTH];
        let mut count = [0; 1];

        let chunks: [&mut [u8]; 6] = [
            &mut dmx,
            &mut type_code,
            &mut lowest_feed_id,
            &mut highest_feed_id,
            &mut xor,
            &mut count,
        ];

        let mut offset: usize = 0;

        for chunk in chunks {
            let len = chunk.len();
            chunk.copy_from_slice(&bytes[offset..offset + len]);
            offset += len;
        }

        if !dmx.eq(&Self::GOSET_DMX) {
            return Err(GOSetClaimError::InvalidDMX);
        }

        if !type_code.eq(&[b'c']) {
            return Err(GOSetClaimError::InvalidTypeCode);
        }

        Ok(GOSetClaim {
            lowest_feed_id: FeedId(lowest_feed_id),
            highest_feed_id: FeedId(highest_feed_id),
            xor: GOSetXor(xor),
            count: count[0],
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
        println!("encoded claim:\n {:?}", claim);

        let result = GOSetClaim::decode(&claim).unwrap();
        println!("decoded claim:\n {:?}", result);

        assert_eq!(&result.lowest_feed_id, go_set.lowest_feed_id().unwrap());
        assert_eq!(&result.highest_feed_id, go_set.highest_feed_id().unwrap());
        assert_eq!(result.xor, go_set.xor());
        assert_eq!(result.count, go_set.count());
    }

    #[test]
    fn claim_wrong_dmx() {
        let feed_a = FeedId([255; 32]);
        let go_set = GOSet::new(&[feed_a]);

        let mut claim = GOSetClaim::encode(&go_set);
        let wrong_dmx = [6; 7];
        claim[0..7].copy_from_slice(&wrong_dmx);
        println!("claim with incorrect dmx:\n {:?}", claim);

        match GOSetClaim::decode(&claim) {
            Err(GOSetClaimError::InvalidDMX) => {} // passed
            _ => panic!("Expected InvalidDMX error"),
        }
    }

    #[test]
    fn claim_wrong_type_code() {
        let feed_a = FeedId([255; 32]);
        let go_set = GOSet::new(&[feed_a]);

        let mut claim = GOSetClaim::encode(&go_set);
        let wrong_type_code = [b'W'];
        claim[7..8].copy_from_slice(&wrong_type_code);
        println!("claim with incorrect type_code:\n {:?}", claim);

        match GOSetClaim::decode(&claim) {
            Err(GOSetClaimError::InvalidTypeCode) => {} // passed
            _ => panic!("Expected InvalidTypeCode error"),
        }
    }
}
