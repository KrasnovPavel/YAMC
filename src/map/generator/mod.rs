pub mod biome;
mod noise_maps;
mod chunk;

use bevy::prelude::{Resource, warn, info};

use crate::map::blocks;
use crate::map::generator::biome::*;
use noise_maps::*;
use crate::map::blocks::BlockType;

pub use chunk::*;

pub const DEBUG_WORLD_SCALE: usize = 1;
pub const CHUNK_RESOLUTION: usize = 16 / DEBUG_WORLD_SCALE;
pub const CUBE_SIDE: f32 = 1.0 / DEBUG_WORLD_SCALE as f32;

const CHUNK_NOISE_BASE_BOUNDS: f64 = 10.0 / 256.0 * CHUNK_RESOLUTION as f64;
const MIN_ZOOM: f64 = 0.01 * 64.0 / CHUNK_RESOLUTION as f64;

#[derive(Resource)]
pub struct Generator {
    pub(crate) seed: u32,
}

impl Generator {
    pub fn get_chunk(&self, ch_x: i32, ch_z: i32) -> Chunk {
        use std::time::Instant;
        let now = Instant::now();

        let base_heights = HeightMap::new(ch_x, ch_z, MIN_ZOOM, self.seed);
        let biome_map = BiomeMap::new(ch_x, ch_z, MIN_ZOOM * 2.0, self.seed - 100, &base_heights);
        let topping_map = ToppingMap::new(ch_x, ch_z, MIN_ZOOM, self.seed - 200, &base_heights, &biome_map);
        let cave_map = CaveMap::new(ch_x, ch_z, MIN_ZOOM * 5.0, self.seed, &biome_map);
        let resource_map = ResourceMap::new(ch_x, ch_z, MIN_ZOOM * 5.0, self.seed);
        let noise_elapsed = now.elapsed();

        let mut result = Chunk::new(ch_x, ch_z);
        let mut i = 0;
        for z in 0..CHUNK_RESOLUTION {
            for x in 0..CHUNK_RESOLUTION {
                for y in 26..27 {
                    result.set(x, y, z, Some(BlockType::UNBREAKABLE));
                    i += 1;
                }

                let by = base_heights.get(x as i32, z as i32);
                for y in 27..by {
                    if cave_map.get(x as i32, y, z as i32) {
                        continue;
                    }
                    result.set(x, y as usize, z, Some(resource_map.get(x as i32, y, z as i32)));
                    i += 1;
                }

                let biome = biome_map.get(x as i32, z as i32);
                let ty = topping_map.get(x as i32, z as i32);
                for y in by..=ty {
                    if cave_map.get(x as i32, y, z as i32)
                        && biome != Biome::Ocean
                        && biome != Biome::FrozenOcean {
                        continue;
                    }

                    let mut block = match biome {
                        Biome::Tundra => BlockType::ICE,
                        Biome::Plains => BlockType::DIRT,
                        Biome::Forest => BlockType::FOREST_DIRT,
                        Biome::Desert => BlockType::SAND,
                        Biome::Mountain => BlockType::STONE,
                        Biome::IcePike => BlockType::ICE,
                        Biome::FrozenOcean => BlockType::ICE,
                        Biome::Ocean => BlockType::WATER,
                    };

                    if block == BlockType::ICE && biome == Biome::FrozenOcean && y < ty - 2 {
                        block = BlockType::WATER;
                    }

                    result.set(x, y as usize, z, Some(block));
                    i += 1;
                }
            }
        }

        let elapsed = now.elapsed();
        info!("Chunk ({ch_x}, {ch_z}) generated: {i} cubes. Noise generation took {noise_elapsed:.2?}. Total generation time: {elapsed:.2?}.");
        result
    }
}