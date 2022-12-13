use noise::core::perlin::perlin_3d;
use noise::permutationtable::PermutationTable;
use crate::map::blocks::BlockType;
use crate::map::generator::noise_maps::Noise3D;
use super::BiomeMap;

const IRON_RANGE: (f64, f64) = (-0.5, -0.49);
const COPPER_RANGE: (f64, f64) = (0.0, 0.01);
const COAL_RANGE: (f64, f64) = (0.5, 0.51);

pub struct ResourceMap {
    perm_table: PermutationTable,
    ch_x: f64,
    ch_z: f64,
    zoom: f64,
}

impl ResourceMap {
    pub fn new(x: i32, z: i32, zoom: f64, seed: u32) -> ResourceMap {
        ResourceMap {
            perm_table: PermutationTable::new(seed),
            ch_x: x as f64 * zoom,
            ch_z: z as f64 * zoom,
            zoom,
        }
    }
}

impl Noise3D<BlockType> for ResourceMap {
    fn get(&self, x: i32, y: u8, z: i32) -> BlockType {
        let (fx, fy, fz) = self.get_pos(x, y, z);
        let value = perlin_3d([fx, fy, fz], &self.perm_table);

        match value {
            _ if value > IRON_RANGE.0 && value < IRON_RANGE.1 => BlockType::IRON,
            _ if value > COPPER_RANGE.0 && value < COPPER_RANGE.1 => BlockType::COPPER,
            _ if value > COAL_RANGE.0 && value < COAL_RANGE.1 => BlockType::COAL,
            _ => BlockType::STONE,
        }
    }

    fn get_zoom(&self) -> f64 {
        self.zoom
    }

    fn get_chunk_pos(&self) -> (f64, f64) {
        (self.ch_x, self.ch_z)
    }
}
