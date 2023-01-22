pub mod biome;
mod noise_maps;

use bevy::prelude::{Resource, warn, info};

use crate::map::generator::biome::*;
use noise_maps::*;
use crate::map::chunk::{BlockType, Chunk};

pub const CUBE_SIDE: f32 = 1.0 as f32;

const CHUNK_NOISE_BASE_BOUNDS: f64 = 10.0 / 256.0 * Chunk::WIDTH as f64;
const MIN_ZOOM: f64 = 0.01 * 64.0 / Chunk::WIDTH as f64;

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

        let mut chunk = Chunk::new();
        let mut i = 0;
        unsafe {
            for z in 0..Chunk::WIDTH {
                for x in 0..Chunk::LENGTH {
                    for y in 0..1 {
                        chunk.spawn_block_unchecked(x, y, z, &BlockType::UNBREAKABLE)
                            .expect("Attempt to spawn block in occupied location while map generation");
                        i += 1;
                    }

                    let by = base_heights.get(x as i32, z as i32);
                    for y in 2..by {
                        if cave_map.get(x as i32, y, z as i32) {
                            continue;
                        }
                        chunk.spawn_block_unchecked(x, y as usize, z, resource_map.get(x as i32, y, z as i32))
                            .expect("Attempt to spawn block in occupied location while map generation");
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
                            Biome::Tundra => &BlockType::ICE,
                            Biome::Plains => &BlockType::DIRT,
                            Biome::Forest => &BlockType::FOREST_DIRT,
                            Biome::Desert => &BlockType::SAND,
                            Biome::Mountain => &BlockType::STONE,
                            Biome::IcePike => &BlockType::ICE,
                            Biome::FrozenOcean => &BlockType::ICE,
                            Biome::Ocean => &BlockType::WATER,
                        };

                        if block == &BlockType::ICE && biome == Biome::FrozenOcean && y < ty - 2 {
                            block = &BlockType::WATER;
                        }

                        chunk.spawn_block_unchecked(x, y as usize, z, block)
                            .expect("Attempt to spawn block in occupied location while map generation");
                        i += 1;
                    }
                }
            }
        }

        let elapsed = now.elapsed();
        info!("Chunk ({ch_x}, {ch_z}) generated: {i} cubes. Noise generation took {noise_elapsed:.2?}. Total generation time: {elapsed:.2?}.");
        chunk
    }
}