use noise::{Perlin, Fbm, utils::{NoiseMap, PlaneMapBuilder, NoiseMapBuilder}};
use crate::map::generator::biome::*;
use crate::map::generator::{CHUNK_NOISE_BASE_BOUNDS, CHUNK_RESOLUTION, DEBUG_WORLD_SCALE};

const VERTICAL_SCALE: f32 = 100.0 / DEBUG_WORLD_SCALE as f32;
const CUBE_HEIGHT: f32 = 1.0 / VERTICAL_SCALE;

pub struct HeightMap {
    noise: NoiseMap,
}

impl HeightMap {
    pub fn new(x: i64, y: i64, zoom: f64, seed: u32) -> Self
    {
        HeightMap {
            noise: generate_noise(x, y, zoom, seed)
        }
    }

    pub fn get(&self, x: usize, y: usize) -> i8 {
        (self.noise.get_value(x, y) as f32 / 2.0 * VERTICAL_SCALE).round() as i8
    }
}

pub struct BiomeMap<'a> {
    temperature: HeightMap,
    height_map: &'a HeightMap,
}

impl BiomeMap<'_> {
    pub fn new<'a>(x: i64, y: i64, zoom: f64, seed: u32, height_map: &'a HeightMap) -> BiomeMap<'a>
    {
        BiomeMap {
            temperature: HeightMap::new(x, y, zoom, seed),
            height_map,
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Biome {
        Biome::from_map(self.temperature.get(x, y), self.height_map.get(x, y))
    }
}

pub struct ToppingMap<'a> {
    desert_noise: HeightMap,
    forest_noise: HeightMap,
    mountain_noise: HeightMap,
    height_map: &'a HeightMap,
    biome_map: &'a BiomeMap<'a>,
}

impl ToppingMap<'_> {
    pub fn new<'a>(x: i64, y: i64, zoom: f64, seed: u32, height_map: &'a HeightMap, biome_map: &'a BiomeMap) -> ToppingMap<'a>
    {
        ToppingMap {
            desert_noise: HeightMap::new(x, y, zoom * 10.0, seed),
            mountain_noise: HeightMap::new(x, y, zoom * 100.0, seed),
            forest_noise: HeightMap::new(x, y, zoom * 50.0, seed),
            height_map,
            biome_map,
        }
    }

    pub fn get(&self, x: usize, y: usize) -> i8 {
        match self.biome_map.get(x, y) {
            Biome::Tundra => self.forest_noise.get(x, y).abs() / 20 + self.height_map.get(x, y),
            Biome::Plains => self.desert_noise.get(x, y).abs() / 20 + self.height_map.get(x, y),
            Biome::Forest => self.forest_noise.get(x, y).abs() / 20 + self.height_map.get(x, y),
            Biome::Desert => self.desert_noise.get(x, y).abs() / 20 + self.height_map.get(x, y),
            Biome::Mountain => self.mountain_noise.get(x, y).abs() / 20 + self.height_map.get(x, y),
            Biome::IcePike => self.mountain_noise.get(x, y).abs() / 20 + self.height_map.get(x, y),
            Biome::FrozenOcean => OCEAN_HEIGHT + self.desert_noise.get(x, y).abs() / 20,
            Biome::Ocean => OCEAN_HEIGHT,
        }
    }
}

fn generate_noise(x: i64, y: i64, zoom: f64, seed: u32) -> NoiseMap {
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