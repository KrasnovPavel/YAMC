mod block;
mod block_type;
pub mod chunk_coordinates;
mod chunk_interaction_plugin;

use std::iter::{Enumerate, Map};
use std::slice::{Iter, IterMut};
use bevy::prelude::*;
pub use block::*;
pub use block_type::*;

#[derive(Component, Clone)]
pub struct Chunk {
    blocks: Vec<Option<Block>>,
    pub is_updated: bool,
    amount_of_blocks: usize,
}

#[derive(Debug)]
pub struct SpawnInOccupiedSpaceError();

#[derive(Debug)]
pub struct PositionNotInChunkError();

impl Chunk {
    pub const WIDTH: u8 = 32; // x
    pub const LENGTH: u8 = 32; // z
    pub const HEIGHT: u8 = 32; // y
    pub const SIZE: usize = Self::WIDTH as usize * Self::LENGTH as usize * Self::HEIGHT as usize;

    pub fn new() -> Self {
        Chunk {
            blocks: vec![None; Self::SIZE],
            is_updated: false,
            amount_of_blocks: 0,
        }
    }

    pub unsafe fn spawn_block_unchecked(&mut self, x: i32, y: i32, z: i32, value: &'static BlockType) {
        let pos = Self::pos_to_index(x, y, z);
        let placed_mut = self.blocks.get_unchecked_mut(pos);
        *placed_mut = Some(value.into());
        self.amount_of_blocks += 1;
    }

    pub fn get_amount_of_blocks(&self) -> usize {
        self.amount_of_blocks
    }

    pub fn iter(&self) -> Iter<'_, Option<Block>> {
        self.blocks.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, Option<Block>> {
        self.blocks.iter_mut()
    }

    pub fn iter_with_pos(&self) -> Map<Enumerate<Iter<Option<Block>>>, fn((usize, &Option<Block>)) -> ((i32, i32, i32), &Option<Block>)> {
        self.blocks.iter()
            .enumerate()
            .map(|(i, block)| (Self::index_to_pos(i), block))
    }


    pub fn iter_mut_with_pos(&mut self) -> Map<Enumerate<IterMut<Option<Block>>>, fn((usize, &mut Option<Block>)) -> ((i32, i32, i32), &mut Option<Block>)> {
        self.blocks.iter_mut()
            .enumerate()
            .map(|(i, block)| (Self::index_to_pos(i), block))
    }

    pub fn index_to_pos(i: usize) -> (i32, i32, i32) {
        let x = (i & 0b11111) as i32;
        let y = ((i >> 10) & 0b11111) as i32;
        let z = ((i >> 5) & 0b11111) as i32;
        (x, y, z)
    }

    pub fn pos_to_index(x: i32, y: i32, z: i32) -> usize {
        ((y << 10) | (z << 5) | x) as usize
    }

    pub fn get_block_at(&self, x: i32, y: i32, z: i32) -> Result<&Option<Block>, PositionNotInChunkError> {
        if x < 0 || x >= Self::WIDTH as i32
            || y < 0 || y >= Self::HEIGHT as i32
            || z < 0 || z >= Self::LENGTH as i32 {
            return Err(PositionNotInChunkError());
        }
        unsafe {
            return Ok(self.get_unchecked(x, y, z));
        }
    }
    
    unsafe fn get_unchecked(&self, x: i32, y: i32, z: i32) -> &Option<Block> {
        self.blocks.get_unchecked(Self::pos_to_index(x, y, z))
    }

    unsafe fn get_unchecked_mut(&mut self, x: i32, y: i32, z: i32) -> &mut Option<Block> {
        self.blocks.get_unchecked_mut(Self::pos_to_index(x, y, z))
    }
}
