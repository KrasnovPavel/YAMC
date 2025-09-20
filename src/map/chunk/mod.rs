mod block;
mod block_type;
mod chunk_interaction_plugin;

use std::iter::{Enumerate, Map};
use std::slice::{Iter, IterMut};
use bevy::prelude::*;
pub use block::*;
pub use block_type::*;
use crate::utils::{BlockPos, CHUNK_SIZE};

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
    pub const SIZE: usize = CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE;

    pub fn new() -> Self {
        Chunk {
            blocks: vec![None; Self::SIZE],
            is_updated: false,
            amount_of_blocks: 0,
        }
    }

    pub unsafe fn spawn_block_unchecked(&mut self, pos: &BlockPos, value: &'static BlockType) {
        let placed_mut = self.blocks.get_unchecked_mut(pos.to_index());
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

    pub fn iter_with_pos(&self) -> Map<Enumerate<Iter<Option<Block>>>, fn((usize, &Option<Block>)) -> (BlockPos, &Option<Block>)> {
        self.blocks.iter()
            .enumerate()
            .map(|(i, block)| (BlockPos::from_index(i), block))
    }


    pub fn iter_mut_with_pos(&mut self) -> Map<Enumerate<IterMut<Option<Block>>>, fn((usize, &mut Option<Block>)) -> (BlockPos, &mut Option<Block>)> {
        self.blocks.iter_mut()
            .enumerate()
            .map(|(i, block)| (BlockPos::from_index(i), block))
    }

    pub fn get_block_at(&self, pos: &BlockPos) -> Result<&Option<Block>, PositionNotInChunkError> {
        if !pos.is_valid() {
            return Err(PositionNotInChunkError());
        }
        unsafe {
            Ok(self.get_unchecked(pos))
        }
    }
    
    unsafe fn get_unchecked(&self, pos: &BlockPos) -> &Option<Block> {
        self.blocks.get_unchecked(pos.to_index())
    }

    unsafe fn get_unchecked_mut(&mut self, pos: &BlockPos) -> &mut Option<Block> {
        self.blocks.get_unchecked_mut(pos.to_index())
    }
}
