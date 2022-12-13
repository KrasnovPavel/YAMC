use crate::map::generator::biome::{Biome, OCEAN_HEIGHT};
use super::{Noise2D, HeightMap, BiomeMap};

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
}

impl Noise2D<u8> for ToppingMap<'_> {
    fn get(&self, x: i32, z: i32) -> u8 {
        match self.biome_map.get(x, z) {
            Biome::Tundra      => self.height_map.get(x, z) + self.forest_noise.get(x, z) / 40   + 2,
            Biome::Plains      => self.height_map.get(x, z) + self.desert_noise.get(x, z) / 40   + 2,
            Biome::Forest      => self.height_map.get(x, z) + self.forest_noise.get(x, z) / 40   + 2,
            Biome::Desert      => self.height_map.get(x, z) + self.desert_noise.get(x, z) / 40   + 2,
            Biome::Mountain    => self.height_map.get(x, z) + self.mountain_noise.get(x, z) / 40,
            Biome::IcePike     => self.height_map.get(x, z) + self.mountain_noise.get(x, z) / 40,
            Biome::FrozenOcean => OCEAN_HEIGHT              + self.desert_noise.get(x, z) / 40   + 2,
            Biome::Ocean       => OCEAN_HEIGHT                                                   + 3,
        }
    }
}