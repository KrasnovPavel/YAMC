use std::iter::Flatten;
use std::slice::Iter;
use crate::map::blocks::BlockType;
use super::CHUNK_RESOLUTION;

pub struct Chunk {
    pub x: i32,
    pub y: i32,
    pub cubes: Vec<Vec<Vec<Option<BlockType>>>>,
    amount_of_cubes: usize,
}

impl Chunk {
    pub fn new(x: i32, y: i32) -> Self {
        let mut cubes: Vec<Vec<Vec<Option<BlockType>>>> = Vec::with_capacity(256);
        cubes.resize(256,
                     std::iter::repeat(
                         std::iter::repeat(None)
                             .take(CHUNK_RESOLUTION)
                             .collect())
                         .take(CHUNK_RESOLUTION)
                         .collect());
        Chunk { x, y, cubes, amount_of_cubes: 0 }
    }

    pub fn set(&mut self, x: usize, y: usize, z: usize, value: Option<BlockType>) {
        match (&self.cubes[y][z][x], &value) {
            (Some(_), Some(_)) => {},
            (Some(_), None)    => {self.amount_of_cubes -= 1;},
            (None,    Some(_)) => {self.amount_of_cubes += 1;},
            (None,    None)    => {},
        }

        self.cubes[y][z][x] = value;
    }

    pub fn get(&self, x: usize, y: usize, z: usize) -> Option<BlockType> {
        self.cubes[y][z][x].clone()
    }

    pub fn get_amount_of_cubes(&self) -> usize {
        self.amount_of_cubes
    }
}
