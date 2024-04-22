use common::chunk::Chunk;
use vek::Vec3;

use crate::block::BlockMap;

use super::{atlas::BlockAtlas, vertex::TerrainVertex};

pub fn create_chunk_mesh(
    c: &Chunk,
    block_atlas: &BlockAtlas,
    block_map: &BlockMap,
) -> Vec<TerrainVertex> {
    let mut mesh = vec![];

    for pos in c.iter_pos() {
        let block = c
            .get(pos)
            .expect("there is always a block for a local block pos");

        let block_settings = block_map
            .blocks
            .get(&block)
            .unwrap_or_else(|| panic!("Not settings found for block with id={:#?}", block));

        let offset = pos.map(|f| f as f32);

        let north_texture = block_atlas.get_texture_id(&block_settings.textures.north).unwrap();

        // North
        mesh.push(TerrainVertex::new(
            Vec3::unit_x() + Vec3::unit_y() + Vec3::unit_z() + offset,
            north_texture,
        ));
        mesh.push(TerrainVertex::new(
            Vec3::unit_x() + Vec3::unit_z() + offset,
            north_texture,
        ));
        mesh.push(TerrainVertex::new(
            Vec3::zero() + Vec3::unit_z() + offset,
            north_texture,
        ));
        mesh.push(TerrainVertex::new(
            Vec3::unit_y() + Vec3::unit_z() + offset,
            north_texture,
        ));
        let south_texture = block_atlas.get_texture_id(&block_settings.textures.south).unwrap();

        // South
        mesh.push(TerrainVertex::new(Vec3::unit_y() + offset, south_texture));
        mesh.push(TerrainVertex::new(Vec3::zero() + offset, south_texture));
        mesh.push(TerrainVertex::new(Vec3::unit_x() + offset, south_texture));
        mesh.push(TerrainVertex::new(
            Vec3::unit_x() + Vec3::unit_y() + offset,
            south_texture,
        ));
        let east_texture = block_atlas.get_texture_id(&block_settings.textures.east).unwrap();

        // East
        mesh.push(TerrainVertex::new(
            Vec3::unit_x() + Vec3::unit_y() + offset,
            east_texture,
        ));
        mesh.push(TerrainVertex::new(Vec3::unit_x() + offset, east_texture));
        mesh.push(TerrainVertex::new(
            Vec3::unit_x() + Vec3::unit_z() + offset,
            east_texture,
        ));
        mesh.push(TerrainVertex::new(
            Vec3::unit_x() + Vec3::unit_z() + Vec3::unit_y() + offset,
            east_texture,
        ));

        let west_texture = block_atlas.get_texture_id(&block_settings.textures.west).unwrap();

        // West
        mesh.push(TerrainVertex::new(
            Vec3::unit_z() + Vec3::unit_y() + offset,
            west_texture,
        ));
        mesh.push(TerrainVertex::new(Vec3::unit_z() + offset, west_texture));
        mesh.push(TerrainVertex::new(Vec3::zero() + offset, west_texture));
        mesh.push(TerrainVertex::new(Vec3::unit_y() + offset, west_texture));

        let top_texture = block_atlas.get_texture_id(&block_settings.textures.top).unwrap();

        // Top
        mesh.push(TerrainVertex::new(
            Vec3::unit_z() + Vec3::unit_y() + offset,
            top_texture,
        ));
        mesh.push(TerrainVertex::new(Vec3::unit_y() + offset, top_texture));
        mesh.push(TerrainVertex::new(
            Vec3::unit_y() + Vec3::unit_x() + offset,
            top_texture,
        ));
        mesh.push(TerrainVertex::new(
            Vec3::unit_y() + Vec3::unit_x() + Vec3::unit_z() + offset,
            top_texture,
        ));

        let bottom_texture = block_atlas.get_texture_id(&block_settings.textures.west).unwrap();

        // Bottom
        mesh.push(TerrainVertex::new(Vec3::zero() + offset, bottom_texture));
        mesh.push(TerrainVertex::new(Vec3::unit_z() + offset, bottom_texture));
        mesh.push(TerrainVertex::new(
            Vec3::unit_z() + Vec3::unit_x() + offset,
            bottom_texture,
        ));
        mesh.push(TerrainVertex::new(Vec3::unit_x() + offset, bottom_texture));
    }
    mesh
}
