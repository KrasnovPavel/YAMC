use std::string::String;
use bevy::prelude::Color;

#[derive(Clone)]
pub struct BlockType {
    pub name: &'static str,
    pub color: Color,
}

pub const ICE: BlockType = BlockType {
    name: "Ice",
    color: Color::AQUAMARINE,
};

pub const STONE: BlockType = BlockType {
    name: "Stone",
    color: Color::GRAY,
};

pub const DIRT: BlockType = BlockType {
    name: "Dirt",
    color: Color::Rgba { red: 55.0 / 128.0, green: 33.0 / 128.0, blue: 0.0, alpha: 1.0 },
};

pub const FOREST_DIRT: BlockType = BlockType {
    name: "ForestDirt",
    color: Color::DARK_GREEN,
};

pub const WATER: BlockType = BlockType {
    name: "Water",
    color: Color::NAVY,
};

pub const SAND: BlockType = BlockType {
    name: "Sand",
    color: Color::GOLD,
};

pub const UNBREAKABLE: BlockType = BlockType {
    name: "Unbreakable",
    color: Color::BLACK,
};
