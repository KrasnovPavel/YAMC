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
    pub const WIDTH: usize = 16; // x
    pub const LENGTH: usize = 16; // z
    pub const HEIGHT: usize = 256;
    pub const SIZE: usize = Self::WIDTH * Self::LENGTH * Self::HEIGHT;

    pub fn new() -> Self {
        Chunk {
            blocks: vec![None; Self::SIZE],
            is_updated: false,
            amount_of_blocks: 0,
        }
    }

    pub unsafe fn spawn_block_unchecked(&mut self, x: usize, y: usize, z: usize, value: &'static BlockType)
                                        -> Result<&Option<Block>, SpawnInOccupiedSpaceError> {
        let placed = self.get_unchecked(x as u8, y as u8, z as u8);
        if let None = placed {
            self.amount_of_blocks += 1;
            let placed_mut = self.get_unchecked_mut(x as u8, y as u8, z as u8);
            *placed_mut = Some(value.into());
            return Ok(placed_mut);
        }
        Err(SpawnInOccupiedSpaceError {})
    }

    pub fn spawn(pop_fn: impl Fn(u8, u8, u8) -> Option<&'static BlockType>) -> Self {
        let mut blocks = vec![None; Self::SIZE];
        let mut amount_of_cubes = 0;

        for (i, block) in blocks.iter_mut().enumerate() {
            let (x, y, z) = Self::index_to_pos(i);
            *block = pop_fn(x, y, z).map(|bl| bl.into());
            if let Some(_) = block {
                amount_of_cubes += 1;
            }
        }

        Chunk {
            blocks,
            is_updated: false,
            amount_of_blocks: amount_of_cubes,
        }
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

    pub fn iter_with_pos(&self) -> Map<Enumerate<Iter<Option<Block>>>, fn((usize, &Option<Block>)) -> ((u8, u8, u8), &Option<Block>)> {
        self.blocks.iter()
            .enumerate()
            .map(|(i, block)| (Self::index_to_pos(i), block))
    }


    pub fn iter_mut_with_pos(&mut self) -> Map<Enumerate<IterMut<Option<Block>>>, fn((usize, &mut Option<Block>)) -> ((u8, u8, u8), &mut Option<Block>)> {
        self.blocks.iter_mut()
            .enumerate()
            .map(|(i, block)| (Self::index_to_pos(i), block))
    }


    pub unsafe fn set_unchecked(&mut self, x: u8, y: u8, z: u8, block: Option<Block>) {
        let placed = self.get_unchecked(x, y, z);
        match (placed, block) {
            (None, None) => {}
            (Some(_), None) => self.amount_of_blocks += 1,
            (None, Some(_)) => self.amount_of_blocks += 1,
            (Some(_), Some(_)) => {}
        }

        let placed_mut = self.get_unchecked_mut(x, y, z);
        *placed_mut = block;
    }

    pub fn index_to_pos(i: usize) -> (u8, u8, u8) {
        let x = (i % Self::LENGTH) as u8;
        let y = (i / (Self::WIDTH * Self::LENGTH)) as u8;
        let z = ((i / Self::LENGTH) % Self::WIDTH) as u8;
        (x, y, z)
    }

    pub fn pos_to_index_u8(x: u8, y: u8, z: u8) -> usize {
        y as usize * Self::WIDTH * Self::LENGTH + z as usize * Self::LENGTH + x as usize
    }

    pub fn pos_to_index(x: usize, y: usize, z: usize) -> usize {
        y * Self::WIDTH * Self::LENGTH + z * Self::LENGTH + x
    }

    pub fn get_block_at(&self, x: i32, y: i32, z: i32) -> Result<&Option<Block>, PositionNotInChunkError> {
        if x < 0 || x >= Self::WIDTH as i32
            || y < 0 || y >= Self::HEIGHT as i32
            || z < 0 || z >= Self::LENGTH as i32 {
            return Err(PositionNotInChunkError());
        }
        unsafe {
            return Ok(self.get_unchecked(x as u8, y as u8, z as u8));
        }
    }

    unsafe fn get_unchecked(&self, x: u8, y: u8, z: u8) -> &Option<Block> {
        self.blocks.get_unchecked(Self::pos_to_index_u8(x, y, z))
    }

    unsafe fn get_unchecked_mut(&mut self, x: u8, y: u8, z: u8) -> &mut Option<Block> {
        self.blocks.get_unchecked_mut(Self::pos_to_index_u8(x, y, z))
    }
}

