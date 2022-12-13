use bevy::prelude::Color;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Biome {
    Tundra,
    Plains,
    Forest,
    Desert,
    Mountain,
    IcePike,
    FrozenOcean,
    Ocean,
}

pub const OCEAN_HEIGHT: u8 = 127 - 25;
pub const MOUNTAIN_HEIGHT: u8 = 127 + 70;
pub const ICE_PIKE_HEIGHT: u8 = 127 + 80;
pub const MAX_DESERT_HEIGHT: u8 = 127 + 60;

pub const FREEZE_TEMP: u8 = 127 - 40;
pub const FOREST_TEMP: u8 = 127;
pub const PLAINS_TEMP: u8 = 127 + 40;

impl Biome {
    pub fn from_map(temperature: u8, vertical: u8) -> Biome {
        match (temperature, vertical) {
            (0..FREEZE_TEMP,           0..OCEAN_HEIGHT)                  => Biome::FrozenOcean,
            (FREEZE_TEMP..u8::MAX,     0..OCEAN_HEIGHT)                  => Biome::Ocean,
            (0..FREEZE_TEMP,           _)                                => Biome::Tundra,
            (_,                        MOUNTAIN_HEIGHT..ICE_PIKE_HEIGHT) => Biome::Mountain,
            (_,                        ICE_PIKE_HEIGHT..u8::MAX)         => Biome::IcePike,
            (FREEZE_TEMP..FOREST_TEMP, _)                                => Biome::Forest,
            (FOREST_TEMP..PLAINS_TEMP, _)                                => Biome::Plains,
            (PLAINS_TEMP..u8::MAX,     OCEAN_HEIGHT..MAX_DESERT_HEIGHT)  => Biome::Desert,
            _                                                            => Biome::Forest,
        }
    }
}

impl From<Biome> for Color {
    fn from(value: Biome) -> Self {
        match value {
            Biome::Tundra      => Color::WHITE,
            Biome::Plains      => Color::rgb_u8(147, 183, 104),
            Biome::Forest      => Color::rgb_u8(59, 123, 78),
            Biome::Desert      => Color::rgb_u8(250, 154, 36),
            Biome::Mountain    => Color::GRAY,
            Biome::IcePike     => Color::WHITE,
            Biome::FrozenOcean => Color::rgb_u8(180, 180, 255),
            Biome::Ocean       => Color::BLUE,
        }
    }
}