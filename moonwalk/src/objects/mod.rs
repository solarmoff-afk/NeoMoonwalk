#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct ShaderId(pub u32);

impl From<u32> for ShaderId {
    fn from(id: u32) -> Self {
        Self(id)
    }
}

impl ShaderId {
    pub fn to_u32(self) -> u32 {
        self.0
    }
}