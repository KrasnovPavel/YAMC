pub mod biome;
mod noise_map_wrappers;

use bevy::prelude::{Resource, warn, info};

use crate::map::blocks;
use crate::map::generator::biome::*;
use crate::map::generator::noise_map_wrappers::*;

pub const DEBUG_WORLD_SCALE: usize = 1;
pub const CHUNK_RESOLUTION: usize = 16 / DEBUG_WORLD_SCALE;
pub const CUBE_SIDE: f32 = 1.0 / DEBUG_WORLD_SCALE as f32;

const CHUNK_NOISE_BASE_BOUNDS: f64 = 10.0 / 256.0 * CHUNK_RESOLUTION as f64;
const MIN_ZOOM: f64 = 0.01 * 64.0 / CHUNK_RESOLUTION as f64;

#[derive(Resource)]
pub struct Generator {
    pub(crate) seed: u32,
}

pub struct Chunk {
    pub x: i64,
    pub y: i64,
    pub cubes: Vec<Vec<Vec<(i8, blocks::BlockType)>>>,
}

impl Generator {
    pub fn get_chunk(&self, ch_x: i64, ch_z: i64) -> Chunk {
        use std::time::Instant;
        let now = Instant::now();

        let base_heights = HeightMap::new(ch_x, ch_z, MIN_ZOOM, self.seed);
        let biome_map = BiomeMap::new(ch_x, ch_z, MIN_ZOOM * 2.0, self.seed - 100, &base_heights);
        let topping_map = ToppingMap::new(ch_x, ch_z, MIN_ZOOM, self.seed - 200, &base_heights, &biome_map);
        let noise_elapsed = now.elapsed();

        let mut result = Vec::with_capacity(CHUNK_RESOLUTION);
        result.resize(CHUNK_RESOLUTION, Vec::with_capacity(CHUNK_RESOLUTION));
        let mut i = 0;
        for z in 0..CHUNK_RESOLUTION {
            for x in 0..CHUNK_RESOLUTION {
                result[z].push(Vec::with_capacity(256));
                for y in -101..-100 {
                    result[z][x].push((y, blocks::UNBREAKABLE));
                    i += 1;
                }

                let by = base_heights.get(x, z);
                for y in -100..by {
                    result[z][x].push((y, blocks::STONE));
                    i += 1;
                }

                let biome = biome_map.get(x, z);
                let ty = topping_map.get(x, z);
                for y in by..=ty {
                    i += 1;
                    let block = match biome {
                        Biome::Tundra => blocks::ICE,
                        Biome::Plains => blocks::DIRT,
                        Biome::Forest => blocks::FOREST_DIRT,
                        Biome::Desert => blocks::SAND,
                        Biome::Mountain => blocks::STONE,
                        Biome::IcePike => blocks::ICE,
                        Biome::FrozenOcean => blocks::ICE,
                        Biome::Ocean => blocks::WATER,
                    };
                    result[z][x].push((y, block));
                }
            }
        }

        let elapsed = now.elapsed();
        info!("Chunk ({ch_x}, {ch_z}) generated: {i} cubes, {noise_elapsed:.2?} - {elapsed:.2?}");
        Chunk {
            x: ch_x,
            y: ch_z,
            cubes: result
        }
    }
}