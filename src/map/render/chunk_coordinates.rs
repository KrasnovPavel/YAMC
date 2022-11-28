use bevy::math::Vec3;
use bevy::prelude::Component;

#[derive(Copy, Clone, Component, Debug)]
pub struct ChunkCoordinates {
    pub x: i64,
    pub z: i64,
}

impl ChunkCoordinates {
    pub fn new(x: i64, z: i64) -> Self{
        ChunkCoordinates { x, z }
    }
}

impl From<ChunkCoordinates> for Vec3 {
    fn from(value: ChunkCoordinates) -> Self {
        Vec3::new(value.x as f32, 0.0, value.z as f32)
    }
}