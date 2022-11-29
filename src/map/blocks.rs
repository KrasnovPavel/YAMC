use bevy::prelude::Color;

#[derive(Clone)]
pub struct BlockType {
    id: usize,
    pub name: &'static str,
    pub color: Color,
}

impl PartialEq for BlockType {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for BlockType {

}

impl BlockType {
    pub const ICE: BlockType = BlockType {
        id: 3,
        name: "Ice",
        color: Color::AQUAMARINE,
    };

    pub const STONE: BlockType = BlockType {
        id: 1,
        name: "Stone",
        color: Color::DARK_GRAY,
    };

    pub const DIRT: BlockType = BlockType {
        id: 2,
        name: "Dirt",
        color: Color::GREEN,
    };

    pub const FOREST_DIRT: BlockType = BlockType {
        id: 4,
        name: "ForestDirt",
        color: Color::DARK_GREEN,
    };

    pub const WATER: BlockType = BlockType {
        id: 5,
        name: "Water",
        color: Color::NAVY,
    };

    pub const SAND: BlockType = BlockType {
        id: 6,
        name: "Sand",
        color: Color::GOLD,
    };

    pub const IRON: BlockType = BlockType {
        id: 7,
        name: "Iron",
        color: Color::GRAY,
    };

    pub const COPPER: BlockType = BlockType {
        id: 8,
        name: "Copper",
        color: Color::ORANGE,
    };

    pub const COAL: BlockType = BlockType {
        id: 9,
        name: "Coal",
        color: Color::BLACK,
    };

    pub const UNBREAKABLE: BlockType = BlockType {
        id: 0,
        name: "Unbreakable",
        color: Color::BLACK,
    };
}
