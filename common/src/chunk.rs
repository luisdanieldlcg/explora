use vek::Vec3;

use crate::block::BlockId;

pub struct Chunk {
    blocks: [BlockId; Self::SIZE.x * Self::SIZE.y * Self::SIZE.z],
}

impl Chunk {
    pub const SIZE: Vec3<usize> = Vec3::new(16, 256, 16);

    pub fn flat() -> Self {
        let mut blocks = [BlockId::Air; Self::SIZE.x * Self::SIZE.y * Self::SIZE.z];
        for x in 0..Self::SIZE.x {
            for y in 0..Self::SIZE.y {
                for z in 0..Self::SIZE.z {
                    let index = Self::index(Vec3::new(x as i32, y as i32, z as i32)).unwrap();
                    blocks[index] = match y {
                        0..=32 => BlockId::Stone,
                        33..=254 => BlockId::Dirt,
                        255 => BlockId::Grass,
                        _ => BlockId::Air,
                    };
                }
            }
        }
        Self { blocks }
    }

    pub fn index(pos: Vec3<i32>) -> Option<usize> {
        if Self::out_of_bounds(pos) {
            return None;
        }
        let pos = pos.map(|s| s as usize);
        Some(Self::SIZE.x * Self::SIZE.y * pos.z + Self::SIZE.x * pos.y + pos.x)
    }

    pub fn get(&self, pos: Vec3<i32>) -> Option<BlockId> {
        if pos.is_any_negative() {
            return None;
        }
        Self::index(pos).map(|index| self.blocks[index])
    }

    pub fn out_of_bounds(pos: Vec3<i32>) -> bool {
        pos.is_any_negative()
            || pos.x >= Self::SIZE.x as i32
            || pos.y >= Self::SIZE.y as i32
            || pos.z >= Self::SIZE.z as i32
    }

    pub fn iter_pos(&self) -> ChunkIter {
        ChunkIter {
            index: 0,
            size: Self::SIZE.map(|x| x as u32),
        }
    }
}

pub struct ChunkIter {
    index: u32,
    size: Vec3<u32>,
}

impl Iterator for ChunkIter {
    type Item = Vec3<i32>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.size.product() {
            return None;
        }
        let x = self.index % self.size.x;
        let y = (self.index / self.size.x) % self.size.y;
        let z = self.index / (self.size.x * self.size.y);

        self.index += 1;
        Some(Vec3::new(x, y, z).map(|f| f as i32))
    }
}

#[cfg(test)]
pub mod tests {
    use vek::Vec3;

    use super::Chunk;

    #[test]
    fn index_test() {
        assert_eq!(Chunk::index(Vec3::new(-1, -1, -1)), None);
        assert_eq!(Chunk::index(Vec3::new(256, 256, 256)), None);

        assert_eq!(Chunk::index(Vec3::new(15, 0, 0)), Some(15));
        assert_eq!(Chunk::index(Vec3::new(0, 255, 0)), Some(16 * 255));
    }

    #[test]
    fn pos_iter_test() {
        let chunk = Chunk::flat();
        let expected_length = Chunk::SIZE.product();
        let actual_length = chunk.iter_pos().count();
        assert_eq!(expected_length, actual_length);
        for p in chunk.iter_pos() {
            assert!(!Chunk::out_of_bounds(p));
        }
    }

    #[test]
    fn out_of_bounds_test() {
        assert!(!Chunk::out_of_bounds(Vec3::zero()));
        assert!(Chunk::out_of_bounds(Vec3::new(-1, 0, 0)));
        assert!(Chunk::out_of_bounds(Vec3::new(0, -1, 0)));
        assert!(Chunk::out_of_bounds(Vec3::new(0, 0, -1)));
        assert!(Chunk::out_of_bounds(Vec3::new(256, 0, 0)));
        assert!(Chunk::out_of_bounds(Vec3::new(0, 256, 0)));
        assert!(Chunk::out_of_bounds(Vec3::new(0, 0, 256)));
    }
}
