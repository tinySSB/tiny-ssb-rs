#[derive(Debug, PartialEq)]
pub struct GOSetXor(pub [u8; 32]);

impl GOSetXor {
    pub const LENGTH: usize = 32;

    pub fn encode(&self) -> [u8; 32] {
        self.0.clone()
    }
}
