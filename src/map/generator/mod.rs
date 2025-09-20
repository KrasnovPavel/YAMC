use rayon::iter::ParallelIterator;
pub mod biome;
mod noise_maps;

use bevy::prelude::{Resource, warn, info, Commands};
use rayon::iter::IntoParallelIterator;
use crate::map::generator::biome::*;
use noise_maps::*;
use crate::map::chunk::{BlockType, Chunk};
use crate::utils::{BlockPos, CHUNK_SIZE, CHUNK_SIZE_F64, CHUNK_SIZE_I32};

pub const CUBE_SIDE: f32 = 1.0f32;

const CHUNK_NOISE_BASE_BOUNDS: f64 = 10.0 / 256.0 * CHUNK_SIZE_F64;
const MIN_ZOOM: f64 = 0.01 * 64.0 / CHUNK_SIZE_F64;
const MAP_HEIGHT: usize = 256;

#[derive(Resource)]
pub struct Generator {
    pub(crate) seed: u32,
}

impl Generator {
    pub fn get_chunk_column(&self, ch_x: i32, ch_z: i32) -> Vec<Chunk> {
        use std::time::Instant;
        let now = Instant::now();
        let base_heights = HeightMap::new(ch_x, ch_z, MIN_ZOOM, self.seed);
        let biome_map = BiomeMap::new(ch_x, ch_z, MIN_ZOOM * 2.0, self.seed - 100, &base_heights);
        let topping_map = ToppingMap::new(ch_x, ch_z, MIN_ZOOM, self.seed - 200, &base_heights, &biome_map);
        let cave_map = CaveMap::new(ch_x, ch_z, MIN_ZOOM * 5.0, self.seed, &biome_map);
        let resource_map = ResourceMap::new(ch_x, ch_z, MIN_ZOOM * 5.0, self.seed);
        let noise_elapsed = now.elapsed();

        let chunk_column = (0..(MAP_HEIGHT / CHUNK_SIZE))
            .into_par_iter()
            .map(|ch_y| self.get_chunk(ch_x, ch_y as i32, ch_z, &base_heights, &biome_map, &topping_map, &cave_map, &resource_map))
            .collect();

        let elapsed = now.elapsed();
        info!("Chunk ({ch_x}, {ch_z}) generated. Noise generation took {noise_elapsed:.2?}. Total generation time: {elapsed:.2?}.");
        chunk_column
    }

    fn get_chunk(&self,
                 ch_x: i32, ch_y: i32, ch_z: i32,
                 base_heights: &HeightMap,
                 biome_map: &BiomeMap,
                 topping_map: &ToppingMap,
                 cave_map: &CaveMap,
                 resource_map: &ResourceMap) -> Chunk {
        use std::time::Instant;
        let now = Instant::now();

        let mut chunk = Chunk::new();
        let mut i = 0;
        for z in 0..CHUNK_SIZE_I32 {
            for x in 0..CHUNK_SIZE_I32 {
                let min_y = ch_y * CHUNK_SIZE_I32;
                let map_height = base_heights.get(x, z) as i32;
                let mut height_in_chunk = i32::min(map_height, min_y + CHUNK_SIZE_I32) - min_y;
                let biome = biome_map.get(x, z);
                let ty = topping_map.get(x, z);
                let topping_height = topping_map.get(x, z) as i32;
                let topping_height_in_chunk = i32::min(topping_height, min_y + CHUNK_SIZE_I32) - min_y;

                for y in 0..height_in_chunk {
                    let block_pos = BlockPos::new(x, y, z);
                    if cave_map.get(x, y + min_y, z) {
                        continue;
                    }
                    unsafe {
                        if (y == 0 && ch_y == 0) {
                            chunk.spawn_block_unchecked(&block_pos, &BlockType::UNBREAKABLE);
                        } else {
                            chunk.spawn_block_unchecked(&block_pos, resource_map.get(x, y + min_y, z));
                        }
                    }
                    i += 1;
                }

                if (topping_height_in_chunk < 0) {
                    continue;
                }

                if (height_in_chunk < 0) {
                    height_in_chunk = 0;
                }

                for y in height_in_chunk..topping_height_in_chunk {
                    let block_pos = BlockPos::new(x, y, z);
                    if  biome != Biome::Ocean
                        && biome != Biome::FrozenOcean
                        && cave_map.get(x, y + min_y, z) {
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

                    if block == &BlockType::ICE && biome == Biome::FrozenOcean && y + min_y < ty as i32 - 2 {
                        block = &BlockType::WATER;
                    }

                    unsafe {
                        chunk.spawn_block_unchecked(&block_pos, block);
                    }
                    i += 1;
                }
            }
        }

        let elapsed = now.elapsed();
        info!("Chunk ({ch_x}, {ch_y}, {ch_z}) generated: {i} cubes. Total generation time: {elapsed:.2?}.");
        chunk
    }
}