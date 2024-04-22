#[derive(Debug, Clone, Copy)]
pub enum BlockId {
    Air,
    Dirt,
    Stone,
    Grass,
}

impl BlockId {
    pub const fn is_air(self) -> bool {
        matches!(self, Self::Air)
    }

    pub const fn is_solid(self) -> bool {
        !self.is_air()
    }
}
