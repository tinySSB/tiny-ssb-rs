use crate::feed_id::FeedId;
use crate::go_set::GOSetXor;

#[derive(Debug, PartialEq)]
pub enum GOSetClaimError {
    InvalidDMX,
    InvalidTypeCode,
}

#[derive(Debug)]
pub struct GOSetClaim {
    pub(crate) lowest_feed_id: FeedId,
    pub(crate) highest_feed_id: FeedId,
    pub(crate) xor: GOSetXor,
    pub(crate) count: u8,
}

impl GOSetClaim {
    const GOSET_DMX: [u8; 7] = [0, 1, 2, 3, 4, 5, 6];
    // TODO: calculate actual DMX
    // ```
    // GOSET_DMX_MATERIAL = "tinySSB-0.1 GOset 1"
    // GOSET_DMX = first 7 bytes of SHA256(GOSET_DMX_MATERIAL)
    // ```

    pub fn encode(&self) -> [u8; 105] {
        // NOTE: we're gonna want to make a range of claims
        // 1. how do we deal with the concept of a "subset" of a claim?
        // PG: I think that GoSets should implement methods that let you create new sets from
        // themselves. So GoSet could have a `bisect(&self) -> GoSet` method. If bisect is the
        // right word
        // 2. where does the logic live about comparing a claim with a goset?
        // PG: I implemented PartialEq on GoSet where the right hand side is GoSetClaim.
        // 3. what's the algorithm for the claim-dance?
        // PG: That's gonna live in some new type we haven't made yet
        //
        // PG: I think that if GoSet.lowest_feed_id returns an option then GoSetClaim
        // lowest_feed_id should also be an option. Same for highest obvs.

        // [DMX (7B) | 'c' (1 byte) | lowest FeedId (32 bytes) | highest FeedId (32 bytes) | XOR (32 bytes) | cnt (1 byte) ]
        let mut claim = [0; 105];

        let dmx = Self::GOSET_DMX;
        let type_code: [u8; 1] = [b'c'];
        let lowest_feed_id = self.lowest_feed_id.encode();
        let highest_feed_id = self.highest_feed_id.encode();
        let xor = self.xor.encode();
        let count: [u8; 1] = [self.count];

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
    use crate::go_set::GOSet;

    #[test]
    fn claim_round_trip() {
        let feed_a = FeedId([255; 32]);
        let feed_b = FeedId([1; 32]);
        let feed_c = FeedId([2; 32]);
        let go_set = GOSet::new(&[feed_a, feed_b, feed_c]).unwrap();

        let claim = go_set.create_claim().unwrap();
        let encoded_claim = claim.encode();
        println!("encoded claim:\n {:?}", claim);

        let result = GOSetClaim::decode(&encoded_claim).unwrap();
        println!("decoded claim:\n {:?}", result);

        assert_eq!(&result.lowest_feed_id, go_set.lowest_feed_id().unwrap());
        assert_eq!(&result.highest_feed_id, go_set.highest_feed_id().unwrap());
        assert_eq!(result.xor, go_set.xor());
        assert_eq!(result.count, go_set.count());
    }

    #[test]
    fn claim_wrong_dmx() {
        let feed_a = FeedId([255; 32]);
        let go_set = GOSet::new(&[feed_a]).unwrap();

        let claim = go_set.create_claim().unwrap();
        let mut encoded_claim = claim.encode();
        let wrong_dmx = [6; 7];
        encoded_claim[0..7].copy_from_slice(&wrong_dmx);
        println!("claim with incorrect dmx:\n {:?}", claim);

        match GOSetClaim::decode(&encoded_claim) {
            Err(GOSetClaimError::InvalidDMX) => {} // passed
            _ => panic!("Expected InvalidDMX error"),
        }
    }

    #[test]
    fn claim_wrong_type_code() {
        let feed_a = FeedId([255; 32]);
        let go_set = GOSet::new(&[feed_a]).unwrap();

        let claim = go_set.create_claim().unwrap();
        let mut encoded_claim = claim.encode();
        let wrong_type_code = [b'W'];
        encoded_claim[7..8].copy_from_slice(&wrong_type_code);
        println!("claim with incorrect type_code:\n {:?}", claim);

        match GOSetClaim::decode(&encoded_claim) {
            Err(GOSetClaimError::InvalidTypeCode) => {} // passed
            _ => panic!("Expected InvalidTypeCode error"),
        }
    }
}
