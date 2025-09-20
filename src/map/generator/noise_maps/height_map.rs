use noise::{Fbm, Perlin};
use noise::utils::{NoiseMap, NoiseMapBuilder, PlaneMapBuilder};
use crate::map::generator::{CHUNK_NOISE_BASE_BOUNDS,};
use crate::utils::CHUNK_SIZE;
use super::utils::*;

pub struct HeightMap {
    noise: NoiseMap,
}

impl HeightMap {
    pub fn new(x: i32, z: i32, zoom: f64, seed: u32) -> Self
    {
        HeightMap {
            noise: Self::generate_2d_noise(x, z, zoom, seed)
        }
    }

    fn generate_2d_noise(x: i32, y: i32, zoom: f64, seed: u32) -> NoiseMap {
        let start_x = (x as f64) * CHUNK_NOISE_BASE_BOUNDS * zoom;
        let start_y = (y as f64) * CHUNK_NOISE_BASE_BOUNDS * zoom;
        let end_x = (x as f64 + 1.0) * CHUNK_NOISE_BASE_BOUNDS * zoom;
        let end_y = (y as f64 + 1.0) * CHUNK_NOISE_BASE_BOUNDS * zoom;

        let fbm = Fbm::<Perlin>::new(seed);

        PlaneMapBuilder::<_, 2>::new(fbm)
            .set_size(CHUNK_SIZE, CHUNK_SIZE)
            .set_x_bounds(start_x, end_x)
            .set_y_bounds(start_y, end_y)
            .build()
    }
}

impl Noise2D<u8> for HeightMap {
    fn get(&self, x: i32, z: i32) -> u8 {
        ((self.noise.get_value(x as usize, z as usize) as f32 / 2.0 + 1.0) * VERTICAL_SCALE).round() as u8
    }
}
