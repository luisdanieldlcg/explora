use std::{collections::HashMap, path::Path};

use image::{GenericImage, RgbaImage};

pub struct BlockAtlas {
    pub buf: RgbaImage,
    pub size: u32,
    pub tile_size: u32,
    pub texture_map: HashMap<String, u32>,
}

impl BlockAtlas {
    pub fn new<P: AsRef<Path>>(resource_path: P) -> Self {
        let files = std::fs::read_dir(&resource_path)
            .unwrap()
            .map(|x| x.map(|x| x.path()))
            // filter out anything that does not contain a png
            .filter(|x| {
                x.as_ref()
                    .unwrap()
                    .extension()
                    .map(|x| x == "png")
                    .unwrap_or(false)
            })
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        tracing::info!(?files);

        // the number of tiles per row/column
        let tile_count = ((files.len() + 1) as f32).sqrt().ceil() as u32;

        let first_image = image::open(&files[0]).unwrap();
        let atlas_width = first_image.width() * tile_count;
        let atlas_height = first_image.height() * tile_count;
        let mut buffer = RgbaImage::new(atlas_width, atlas_height);

        // TODO: write default texture

        tracing::info!(
            ?tile_count,
            ?atlas_width,
            ?atlas_height,
            texture_width = first_image.width(),
            texture_height = first_image.height()
        );

        let mut id = 1u32;
        let mut texture_map = HashMap::new();
        for file in &files {
            let texture = match image::open(file) {
                Ok(t) => t,
                Err(e) => {
                    tracing::error!("Failed to read texture: {}", e);
                    continue;
                }
            };
            if texture.width() != first_image.width() || texture.height() != first_image.height() {
                tracing::warn!(
                    "Ignoring texture with invalid size: {}x{} (expected {}x{}).",
                    texture.width(),
                    texture.height(),
                    first_image.width(),
                    first_image.height(),
                );
                continue;
            }

            tracing::info!(?id, file = ?file.display(), "Packing texture");

            let x = (id % tile_count) * first_image.width();
            let y = (id / tile_count) * first_image.height();
            // TODO: check errors
            let _ = buffer.copy_from(&texture, x, y);
            // this is ugly
            texture_map.insert(file.file_stem().unwrap().to_str().unwrap().to_owned(), id);
            id += 1;
        }

        buffer.save("atlas.png").expect("Failed to save atlas");

        Self {
            buf: buffer,
            size: atlas_width,
            tile_size: first_image.width(),
            texture_map,
        }
    }

    pub fn get_texture_id(&self, texture_name: &str) -> Option<u32> {
        self.texture_map.get(texture_name).copied()
    }
}
