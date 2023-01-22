use noise::core::perlin::perlin_3d;
use noise::permutationtable::PermutationTable;
use crate::map::chunk::BlockType;
use crate::map::generator::noise_maps::Noise3D;

const IRON_PROBABILITY: f64 = 0.1;
const COPPER_PROBABILITY: f64 = 0.1;
const COAL_PROBABILITY: f64 = 0.1;

pub struct ResourceMap {
    iron_table: PermutationTable,
    copper_table: PermutationTable,
    coal_table: PermutationTable,
    ch_x: f64,
    ch_z: f64,
    zoom: f64,
}

impl ResourceMap {
    pub fn new(x: i32, z: i32, zoom: f64, seed: u32) -> ResourceMap {
        ResourceMap {
            iron_table: PermutationTable::new(seed),
            copper_table: PermutationTable::new(seed + 9865),
            coal_table: PermutationTable::new(seed + 452),
            ch_x: x as f64 * zoom,
            ch_z: z as f64 * zoom,
            zoom,
        }
    }
}

impl Noise3D<&'static BlockType> for ResourceMap {
    fn get(&self, x: i32, y: u8, z: i32) -> &'static BlockType {
        let (fx, fy, fz) = self.get_pos(x, y, z);

        let iron = (perlin_3d([fx, fy, fz], &self.iron_table) + 1.0) / 2.0;
        if iron < IRON_PROBABILITY {
            return &BlockType::IRON;
        }

        let copper = (perlin_3d([fx, fy, fz], &self.copper_table) + 1.0) / 2.0;
        if copper < COPPER_PROBABILITY {
            return &BlockType::COPPER;
        }

        let coal = (perlin_3d([fx, fy, fz], &self.coal_table) + 1.0) / 2.0;
        if coal < COAL_PROBABILITY {
            return &BlockType::COAL;
        }

        &BlockType::STONE
    }

    fn get_zoom(&self) -> f64 {
        self.zoom
    }

    fn get_chunk_pos(&self) -> (f64, f64) {
        (self.ch_x, self.ch_z)
    }
}
