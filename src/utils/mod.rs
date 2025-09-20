use bevy::math::{IVec3, Vec3};
use bevy::prelude::Component;

pub const CHUNK_SIZE: usize = 32;
pub const CHUNK_SIZE_I32: i32 = CHUNK_SIZE as i32;
pub const CHUNK_SIZE_F32: f32 = CHUNK_SIZE as f32;
pub const CHUNK_SIZE_F64: f64 = CHUNK_SIZE as f64;

/// Block coordinates in world space.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct WorldBlockPos(pub IVec3);

/// Chunk coordinates in world space.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Component)]
pub struct ChunkPos(pub IVec3);

/// Block coordinates in chunk space.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct BlockPos(pub IVec3);

/// Coordinates in bevy world space.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WorldPos(pub Vec3);

impl WorldBlockPos {
    pub fn from (chunk: &ChunkPos, block: &BlockPos) -> WorldBlockPos {
        WorldBlockPos(IVec3::new(
            chunk.0.x * CHUNK_SIZE_I32 + block.0.x,
            chunk.0.y * CHUNK_SIZE_I32 + block.0.y,
            chunk.0.z * CHUNK_SIZE_I32 + block.0.z,
        ))
    }
}

impl ChunkPos {
    pub fn new(x: i32, y: i32, z: i32) -> ChunkPos {
        ChunkPos(IVec3::new(x, y, z))
    }

    pub fn planar_distance(&self, pos: &ChunkPos) -> i32 {
        (self.0.x - pos.0.x).abs() + (self.0.z - pos.0.z).abs()
    }
}

impl From<ChunkPos> for WorldPos {
    fn from(value: ChunkPos) -> Self {
        WorldPos(Vec3::new((value.0.x as f32) * CHUNK_SIZE_F32,
                           (value.0.y as f32) * CHUNK_SIZE_F32 - 100.0,
                           (value.0.z as f32) * CHUNK_SIZE_F32))

    }
}

impl BlockPos {
    pub fn new(x: i32, y: i32, z: i32) -> BlockPos {
        BlockPos(IVec3::new(x, y, z))
    }

    pub fn from_index(i: usize) -> BlockPos {
        let x = (i & 0b11111) as i32;
        let y = ((i >> 10) & 0b11111) as i32;
        let z = ((i >> 5) & 0b11111) as i32;

        BlockPos(IVec3::new(x, y, z))
    }

    pub fn to_index(self) -> usize {
        ((self.0.y << 10) | (self.0.z << 5) | self.0.x) as usize
    }

    pub fn is_valid(&self) -> bool {
        self.0.x >= 0 && self.0.y >= 0 && self.0.z >= 0 &&
            self.0.x < CHUNK_SIZE_I32 && self.0.y < CHUNK_SIZE_I32 && self.0.z < CHUNK_SIZE_I32
    }
}

impl From<WorldBlockPos> for WorldPos {
    fn from(value: WorldBlockPos) -> WorldPos {
        WorldPos(Vec3::new(value.0.x as f32, value.0.y as f32 - 100., value.0.z as f32))
    }
}

impl From<WorldPos> for WorldBlockPos {
    fn from(value: WorldPos) -> Self {
        WorldBlockPos(IVec3::new(value.0.x as i32, value.0.y as i32 + 100, value.0.z as i32))
    }
}

impl From<WorldBlockPos> for ChunkPos {
    fn from(value: WorldBlockPos) -> ChunkPos {
        ChunkPos(IVec3::new(
            if (value.0.x >= 0) {value.0.x / CHUNK_SIZE_I32} else {(value.0.x + 1) / CHUNK_SIZE_I32 - 1},
            if (value.0.y >= 0) {value.0.y / CHUNK_SIZE_I32} else {(value.0.y + 1) / CHUNK_SIZE_I32 - 1},
            if (value.0.z >= 0) {value.0.z / CHUNK_SIZE_I32} else {(value.0.z + 1) / CHUNK_SIZE_I32 - 1},
        ))
    }
}

impl From<WorldBlockPos> for BlockPos {
    fn from(value: WorldBlockPos) -> BlockPos {
        BlockPos(IVec3::new(
            if (value.0.x >= 0) {value.0.x % CHUNK_SIZE_I32} else {(((value.0.x + 1) % CHUNK_SIZE_I32) - 1) + CHUNK_SIZE_I32},
            if (value.0.y >= 0) {value.0.y % CHUNK_SIZE_I32} else {(((value.0.y + 1) % CHUNK_SIZE_I32) - 1) + CHUNK_SIZE_I32},
            if (value.0.z >= 0) {value.0.z % CHUNK_SIZE_I32} else {(((value.0.z + 1) % CHUNK_SIZE_I32) - 1) + CHUNK_SIZE_I32},
        ))
    }
}


