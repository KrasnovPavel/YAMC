use bevy::math::Vec3;
use bevy::prelude::Component;
use crate::map::chunk::Chunk;

#[derive(Copy, Clone, Component, Debug, PartialEq, Eq, Hash)]
pub struct ChunkCoordinates {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl ChunkCoordinates {
    pub fn new(x: i32, y: i32, z: i32) -> Self{
        ChunkCoordinates { x, y, z }
    }

    pub fn distance(&self, x: i32, z: i32) -> i32 {
        (self.x - x).abs() + (self.z - z).abs()
    }

    pub fn global_pos(&self) -> Vec3 {
        Vec3::new((self.x as f32) * Chunk::WIDTH as f32, (self.y as f32) * Chunk::HEIGHT as f32 - 100.0, (self.z as f32) * Chunk::LENGTH as f32)
    }

    pub fn from_global_block_pos(x: i32, y: i32, z: i32) -> (Self, i32, i32, i32) {
        let (ch_x, bx) = Self::from_global_block_pos_axis(x, Chunk::WIDTH as i32);
        let (ch_y, by) = Self::from_global_block_pos_axis(y, Chunk::HEIGHT as i32);
        let (ch_z, bz) = Self::from_global_block_pos_axis(z, Chunk::LENGTH as i32);

        (ChunkCoordinates::new(ch_x, ch_y, ch_z),
         bx, by, bz)
    }

    fn from_global_block_pos_axis(x: i32, size: i32) -> (i32, i32){
        if (x >= 0) {
            (x / size, (x % size))
        } else {
            ((x + 1) / size - 1, ((((x + 1) % size) - 1) + size))
        }
    }

    pub fn get_global_block_pos(&self, dx: i32, dy: i32, dz: i32) -> (i32, i32, i32) {
        (self.x * Chunk::WIDTH as i32 + dx,
         self.y * Chunk::HEIGHT as i32 + dy,
         self.z * Chunk::LENGTH as i32 + dz)
    }
}

impl From<ChunkCoordinates> for Vec3 {
    fn from(value: ChunkCoordinates) -> Self {
        Vec3::new(value.x as f32, 0.0, value.z as f32)
    }
}