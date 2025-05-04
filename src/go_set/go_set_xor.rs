#[derive(Debug, PartialEq)]
pub struct GOSetXor(pub [u8; 32]);

impl GOSetXor {
    pub fn encode(&self) -> [u8; 32] {
        self.0.clone()
    }
}
