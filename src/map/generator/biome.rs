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

pub const OCEAN_HEIGHT: i8 = -25;
pub const MOUNTAIN_HEIGHT: i8 = 70;
pub const ICE_PIKE_HEIGHT: i8 = 80;
pub const MAX_DESERT_HEIGHT: i8 = 60;

pub const FREEZE_TEMP: i8 = -40;
pub const FOREST_TEMP: i8 = 0;
pub const PLAINS_TEMP: i8 = 40;

impl Biome {
    pub fn from_map(temperature: i8, vertical: i8) -> Biome {
        match (temperature, vertical) {
            (i8::MIN..FREEZE_TEMP,     i8::MIN..OCEAN_HEIGHT)            => Biome::FrozenOcean,
            (FREEZE_TEMP..i8::MAX,     i8::MIN..OCEAN_HEIGHT)            => Biome::Ocean,
            (i8::MIN..FREEZE_TEMP,     _)                                => Biome::Tundra,
            (_,                        MOUNTAIN_HEIGHT..ICE_PIKE_HEIGHT) => Biome::Mountain,
            (_,                        ICE_PIKE_HEIGHT..i8::MAX)         => Biome::IcePike,
            (FREEZE_TEMP..FOREST_TEMP, _)                                => Biome::Forest,
            (FOREST_TEMP..PLAINS_TEMP, _)                                => Biome::Plains,
            (PLAINS_TEMP..i8::MAX,     OCEAN_HEIGHT..MAX_DESERT_HEIGHT)  => Biome::Desert,
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