use noise::utils::NoiseMap;
use super::utils::*;

pub struct HeightMap {
    noise: NoiseMap,
}

impl HeightMap {
    pub fn new(x: i32, z: i32, zoom: f64, seed: u32) -> Self
    {
        HeightMap {
            noise: generate_2d_noise(x, z, zoom, seed)
        }
    }
}

impl Noise2D<i8> for HeightMap {
    fn get(&self, x: i32, z: i32) -> i8 {
        (self.noise.get_value(x as usize, z as usize) as f32 / 2.0 * VERTICAL_SCALE).round() as i8
    }
}
