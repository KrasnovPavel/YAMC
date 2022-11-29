use bevy::math::Vec3;
use bevy::prelude::Component;

#[derive(Copy, Clone, Component, Debug)]
pub struct ChunkCoordinates {
    pub x: i32,
    pub z: i32,
}

impl ChunkCoordinates {
    pub fn new(x: i32, z: i32) -> Self{
        ChunkCoordinates { x, z }
    }

    pub fn distance(&self, x: i32, z: i32) -> i32 {
        (self.x - x).abs() + (self.z - z).abs()
    }
}

impl From<ChunkCoordinates> for Vec3 {
    fn from(value: ChunkCoordinates) -> Self {
        Vec3::new(value.x as f32, 0.0, value.z as f32)
    }
}