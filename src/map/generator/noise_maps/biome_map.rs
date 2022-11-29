use crate::map::generator::biome::Biome;
use crate::map::generator::noise_maps::utils::Noise2D;
use super::HeightMap;

pub struct BiomeMap<'a> {
    temperature: HeightMap,
    height_map: &'a HeightMap,
}

impl BiomeMap<'_> {
    pub fn new(x: i32, z: i32, zoom: f64, seed: u32, height_map: &HeightMap) -> BiomeMap
    {
        BiomeMap {
            temperature: HeightMap::new(x, z, zoom, seed),
            height_map,
        }
    }
}

impl Noise2D<Biome> for BiomeMap<'_> {
    fn get(&self, x: i32, z: i32) -> Biome {
        Biome::from_map(self.temperature.get(x, z), self.height_map.get(x, z))
    }
}
