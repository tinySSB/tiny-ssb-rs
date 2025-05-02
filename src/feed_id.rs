pub struct FeedId([u8; 32]);
impl FeedId {
    pub fn encode(&self) -> [u8; 32] {
        self.0.clone()
    }
}

