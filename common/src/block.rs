#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

impl From<&String> for BlockId {
    fn from(value: &String) -> Self {
        match value.to_lowercase().as_str() {
            "dirt" => BlockId::Dirt,
            "stone" => BlockId::Stone,
            "grass" => BlockId::Grass,
            _ => BlockId::Air,
        }
    }
}
