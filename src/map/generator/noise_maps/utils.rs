use noise::{Fbm, Perlin, utils::{NoiseMap, NoiseMapBuilder, PlaneMapBuilder}};
use crate::map::generator::{CHUNK_NOISE_BASE_BOUNDS, CHUNK_RESOLUTION, DEBUG_WORLD_SCALE};

pub const VERTICAL_SCALE: f32 = 100.0 / DEBUG_WORLD_SCALE as f32;

pub trait Noise2D<T> {
    fn get(&self, x: i32, z: i32) -> T;
}

pub trait Noise3D<T> {
    fn get(&self, x: i32, y: i8, z: i32) -> T;

    fn get_zoom(&self) -> f64;
    fn get_chunk_pos(&self) -> (f64, f64);

    fn get_pos(&self, x: i32, y: i8, z: i32) -> (f64, f64, f64) {
        let (ch_x, ch_z) = self.get_chunk_pos();
        let fx = (x as f64) * self.get_zoom() / CHUNK_RESOLUTION as f64 + ch_x;
        let fy = (y as f64) * self.get_zoom() / 2.0;
        let fz = (z as f64) * self.get_zoom() / CHUNK_RESOLUTION as f64 + ch_z;

        (fx, fy, fz)
    }
}

pub fn generate_2d_noise(x: i32, y: i32, zoom: f64, seed: u32) -> NoiseMap {
    let start_x = (x as f64) * CHUNK_NOISE_BASE_BOUNDS * zoom;
    let start_y = (y as f64) * CHUNK_NOISE_BASE_BOUNDS * zoom;
    let end_x = (x as f64 + 1.0) * CHUNK_NOISE_BASE_BOUNDS * zoom;
    let end_y = (y as f64 + 1.0) * CHUNK_NOISE_BASE_BOUNDS * zoom;

    let fbm = Fbm::<Perlin>::new(seed);

    PlaneMapBuilder::<_, 2>::new(fbm)
        .set_size(CHUNK_RESOLUTION, CHUNK_RESOLUTION)
        .set_x_bounds(start_x, end_x)
        .set_y_bounds(start_y, end_y)
        .build()
}
