use noise::core::perlin::perlin_3d;
use noise::permutationtable::PermutationTable;
use crate::map::generator::biome::Biome;
use super::{BiomeMap, Noise2D, Noise3D};

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
}

impl Noise3D<bool> for CaveMap<'_> {
    fn get(&self, x: i32, y: u8, z: i32) -> bool {
        if self.biome_map.get(x, z) == Biome::Ocean {
            return false;
        }

        let (fx, fy, fz) = self.get_pos(x, y, z);
        let res1 = perlin_3d([fx, fy, fz], &self.perm_table1);
        let res2 = perlin_3d([fx, fy, fz], &self.perm_table2);
        let value = (res1 + res2 + 2.0) / 4.0;

        return value < CAVE_PROBABILITY
    }

    fn get_zoom(&self) -> f64 {
        self.zoom
    }

    fn get_chunk_pos(&self) -> (f64, f64) {
        (self.ch_x, self.ch_z)
    }
}
