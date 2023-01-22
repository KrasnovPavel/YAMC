use bevy::math::Vec3;
use bevy::prelude::Component;
use crate::map::chunk::Chunk;

#[derive(Copy, Clone, Component, Debug, PartialEq, Eq, Hash)]
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

    pub fn global_pos(&self) -> Vec3 {
        Vec3::new((self.x as f32) * Chunk::WIDTH as f32, -100.0, (self.z as f32) * Chunk::LENGTH as f32)
    }

    pub fn from_voxel_coordinates(x: i32, y: i32, z: i32) -> (ChunkCoordinates, (i32, i32, i32)) {
        let cx = x / Chunk::WIDTH as i32;
        let cz = z / Chunk::LENGTH as i32;

        let mut dx = x % Chunk::WIDTH as i32;
        if dx < 0 {
            dx = Chunk::WIDTH as i32 + dx;
        }
        let mut dz = z % Chunk::LENGTH as i32;
        if dz < 0 {
            dz = Chunk::LENGTH as i32 + dz;
        }
        (ChunkCoordinates::new(cx, cz), (dx, y + 100, dz))
    }
}

impl From<ChunkCoordinates> for Vec3 {
    fn from(value: ChunkCoordinates) -> Self {
        Vec3::new(value.x as f32, 0.0, value.z as f32)
    }
}