use common::chunk::Chunk;
use vek::Vec3;

use super::vertex::TerrainVertex;

pub fn create_chunk_mesh(c: &Chunk) -> Vec<TerrainVertex> {
    let mut mesh = vec![];
    let id = 1;
    for pos in c.iter_pos() {
        let offset = pos.map(|f| f as f32);
        // North
        mesh.push(TerrainVertex::new(
            Vec3::unit_x() + Vec3::unit_y() + Vec3::unit_z() + offset,
            id,
        ));
        mesh.push(TerrainVertex::new(
            Vec3::unit_x() + Vec3::unit_z() + offset,
            id,
        ));
        mesh.push(TerrainVertex::new(
            Vec3::zero() + Vec3::unit_z() + offset,
            id,
        ));
        mesh.push(TerrainVertex::new(
            Vec3::unit_y() + Vec3::unit_z() + offset,
            id,
        ));

        // South
        mesh.push(TerrainVertex::new(Vec3::unit_y() + offset, id));
        mesh.push(TerrainVertex::new(Vec3::zero() + offset, id));
        mesh.push(TerrainVertex::new(Vec3::unit_x() + offset, id));
        mesh.push(TerrainVertex::new(
            Vec3::unit_x() + Vec3::unit_y() + offset,
            id,
        ));

        // East
        mesh.push(TerrainVertex::new(
            Vec3::unit_x() + Vec3::unit_y() + offset,
            id,
        ));
        mesh.push(TerrainVertex::new(Vec3::unit_x() + offset, id));
        mesh.push(TerrainVertex::new(
            Vec3::unit_x() + Vec3::unit_z() + offset,
            id,
        ));
        mesh.push(TerrainVertex::new(
            Vec3::unit_x() + Vec3::unit_z() + Vec3::unit_y() + offset,
            id,
        ));
        // West
        mesh.push(TerrainVertex::new(
            Vec3::unit_z() + Vec3::unit_y() + offset,
            id,
        ));
        mesh.push(TerrainVertex::new(Vec3::unit_z() + offset, id));
        mesh.push(TerrainVertex::new(Vec3::zero() + offset, id));
        mesh.push(TerrainVertex::new(Vec3::unit_y() + offset, id));
        // Top
        mesh.push(TerrainVertex::new(
            Vec3::unit_z() + Vec3::unit_y() + offset,
            id,
        ));
        mesh.push(TerrainVertex::new(Vec3::unit_y() + offset, id));
        mesh.push(TerrainVertex::new(
            Vec3::unit_y() + Vec3::unit_x() + offset,
            id,
        ));
        mesh.push(TerrainVertex::new(
            Vec3::unit_y() + Vec3::unit_x() + Vec3::unit_z() + offset,
            id,
        ));
        // Bottom
        mesh.push(TerrainVertex::new(Vec3::zero() + offset, id));
        mesh.push(TerrainVertex::new(Vec3::unit_z() + offset, id));
        mesh.push(TerrainVertex::new(
            Vec3::unit_z() + Vec3::unit_x() + offset,
            id,
        ));
        mesh.push(TerrainVertex::new(Vec3::unit_x() + offset, id));
    }
    mesh
}
