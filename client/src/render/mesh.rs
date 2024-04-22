use common::chunk::Chunk;
use vek::Vec3;

use crate::vertex::TerrainVertex;

pub fn create_chunk_mesh(c: &Chunk) -> Vec<TerrainVertex> {
    let mut mesh = vec![];

    for pos in c.iter_pos() {
        let offset = pos.map(|f| f as f32);
        // North
        mesh.push(TerrainVertex::new(
            Vec3::unit_x() + Vec3::unit_y() + Vec3::unit_z() + offset,
        ));
        mesh.push(TerrainVertex::new(Vec3::unit_x() + Vec3::unit_z() + offset));
        mesh.push(TerrainVertex::new(Vec3::zero() + Vec3::unit_z() + offset));
        mesh.push(TerrainVertex::new(Vec3::unit_y() + Vec3::unit_z() + offset));

        // South
        mesh.push(TerrainVertex::new(Vec3::unit_y() + offset));
        mesh.push(TerrainVertex::new(Vec3::zero() + offset));
        mesh.push(TerrainVertex::new(Vec3::unit_x() + offset));
        mesh.push(TerrainVertex::new(Vec3::unit_x() + Vec3::unit_y() + offset));

        // East
        mesh.push(TerrainVertex::new(Vec3::unit_x() + Vec3::unit_y() + offset));
        mesh.push(TerrainVertex::new(Vec3::unit_x() + offset));
        mesh.push(TerrainVertex::new(Vec3::unit_x() + Vec3::unit_z() + offset));
        mesh.push(TerrainVertex::new(
            Vec3::unit_x() + Vec3::unit_z() + Vec3::unit_y() + offset,
        ));
        // West
        mesh.push(TerrainVertex::new(Vec3::unit_z() + Vec3::unit_y() + offset));
        mesh.push(TerrainVertex::new(Vec3::unit_z() + offset));
        mesh.push(TerrainVertex::new(Vec3::zero() + offset));
        mesh.push(TerrainVertex::new(Vec3::unit_y() + offset));
        // Top
        mesh.push(TerrainVertex::new(Vec3::unit_z() + Vec3::unit_y() + offset));
        mesh.push(TerrainVertex::new(Vec3::unit_y() + offset));
        mesh.push(TerrainVertex::new(Vec3::unit_y() + Vec3::unit_x() + offset));
        mesh.push(TerrainVertex::new(
            Vec3::unit_y() + Vec3::unit_x() + Vec3::unit_z() + offset,
        ));
        // Bottom
        mesh.push(TerrainVertex::new(Vec3::zero() + offset));
        mesh.push(TerrainVertex::new(Vec3::unit_z() + offset));
        mesh.push(TerrainVertex::new(Vec3::unit_z() + Vec3::unit_x() + offset));
        mesh.push(TerrainVertex::new(Vec3::unit_x() + offset));
    }
    mesh
}
