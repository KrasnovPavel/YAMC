use bevy::log::info;
use noise::{Perlin, Fbm, utils::{NoiseMap, PlaneMapBuilder, NoiseMapBuilder}};
use noise::core::perlin::perlin_3d;
use noise::permutationtable::PermutationTable;
use crate::map::generator::biome::*;
use crate::map::generator::{CHUNK_NOISE_BASE_BOUNDS, CHUNK_RESOLUTION, DEBUG_WORLD_SCALE};

const VERTICAL_SCALE: f32 = 100.0 / DEBUG_WORLD_SCALE as f32;
const CUBE_HEIGHT: f32 = 1.0 / VERTICAL_SCALE;

pub struct HeightMap {
    noise: NoiseMap,
}

impl HeightMap {
    pub fn new(x: i32, z: i32, zoom: f64, seed: u32) -> Self
    {
        HeightMap {
            noise: generate_noise(x, z, zoom, seed)
        }
    }

    pub fn get(&self, x: i32, z: i32) -> i8 {
        (self.noise.get_value(x as usize, z as usize) as f32 / 2.0 * VERTICAL_SCALE).round() as i8
    }
}

pub struct BiomeMap<'a> {
    temperature: HeightMap,
    height_map: &'a HeightMap,
}

impl BiomeMap<'_> {
    pub fn new<'a>(x: i32, z: i32, zoom: f64, seed: u32, height_map: &'a HeightMap) -> BiomeMap<'a>
    {
        BiomeMap {
            temperature: HeightMap::new(x, z, zoom, seed),
            height_map,
        }
    }

    pub fn get(&self, x: i32, z: i32) -> Biome {
        Biome::from_map(self.temperature.get(x, z), self.height_map.get(x, z))
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
    pub fn new<'a>(x: i32, z: i32, zoom: f64, seed: u32, height_map: &'a HeightMap, biome_map: &'a BiomeMap) -> ToppingMap<'a>
    {
        ToppingMap {
            desert_noise: HeightMap::new(x, z, zoom * 10.0, seed),
            mountain_noise: HeightMap::new(x, z, zoom * 100.0, seed),
            forest_noise: HeightMap::new(x, z, zoom * 50.0, seed),
            height_map,
            biome_map,
        }
    }

    pub fn get(&self, x: i32, z: i32) -> i8 {
        match self.biome_map.get(x, z) {
            Biome::Tundra => self.forest_noise.get(x, z).abs() / 20 + self.height_map.get(x, z) + 2,
            Biome::Plains => self.desert_noise.get(x, z).abs() / 20 + self.height_map.get(x, z) + 2,
            Biome::Forest => self.forest_noise.get(x, z).abs() / 20 + self.height_map.get(x, z) + 2,
            Biome::Desert => self.desert_noise.get(x, z).abs() / 20 + self.height_map.get(x, z) + 2,
            Biome::Mountain => self.mountain_noise.get(x, z).abs() / 20 + self.height_map.get(x, z),
            Biome::IcePike => self.mountain_noise.get(x, z).abs() / 20 + self.height_map.get(x, z),
            Biome::FrozenOcean => OCEAN_HEIGHT + self.desert_noise.get(x, z).abs() / 20 + 2,
            Biome::Ocean => OCEAN_HEIGHT + 2,
        }
    }
}

const CAVE_PROBABILITY: f64 = 0.3;

pub struct CaveMap<'a> {
    perm_table1: PermutationTable,
    perm_table2: PermutationTable,
    biome_map: &'a BiomeMap<'a>,
    ch_x: f64,
    ch_z: f64,
    zoom: f64,
}

impl CaveMap<'_> {
    pub fn new<'a> (x: i32, z: i32, zoom: f64, seed: u32, biome_map: &'a BiomeMap<'a>) -> CaveMap<'a> {
        CaveMap {
            perm_table1: PermutationTable::new(seed),
            perm_table2: PermutationTable::new(seed - 758),
            biome_map,
            ch_x: x as f64 * zoom,
            ch_z: z as f64 * zoom,
            zoom,
        }
    }

    pub fn get(&self, x: i32, y: i8, z: i32) -> bool {
        if self.biome_map.get(x, z) == Biome::Ocean {
            return false;
        }

        let fx = (x as f64) * self.zoom / CHUNK_RESOLUTION as f64 + self.ch_x;
        let fy = (y as f64) * self.zoom / 2.0;
        let fz = (z as f64) * self.zoom / CHUNK_RESOLUTION as f64 + self.ch_z;
        let res1 = perlin_3d([fx, fy, fz], &self.perm_table1);
        let res2 = perlin_3d([fx, fy, fz], &self.perm_table2);
        let value = (res1 + res2 + 2.0) / 4.0;

        return value < CAVE_PROBABILITY
    }
}

fn generate_noise(x: i32, y: i32, zoom: f64, seed: u32) -> NoiseMap {
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