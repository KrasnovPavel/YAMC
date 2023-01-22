use bevy::prelude::*;
use itertools::Itertools;
use crate::map::chunk::{Block, BlockType, Chunk};
use crate::map::chunk::chunk_coordinates::ChunkCoordinates;

#[derive(Component, Eq, PartialEq)]
pub struct Player(usize);

fn setup_players(query: Query<Entity, With<Camera>>, mut commands: Commands) {
    for (i, e) in query.iter().enumerate() {
        println!("Player was added");
        commands
            .entity(e)
            .insert(Player(i));
    }
}

fn interact_with_chunk(players: Query<&Transform, With<Player>>,
                       mut chunks: Query<(&ChunkCoordinates, &mut Chunk)>) {
    // let transform_op = players.iter().next();
    // if let None = transform_op {
    //     return;
    // }
    // let transform = transform_op.unwrap();
    //
    // let coords = std::iter::repeat(transform.translation)
    //     .enumerate()
    //     .map(|(i, pos)| pos + transform.forward() * i as f32)
    //     .map(|pos| (pos.x as i32, pos.y as i32, pos.z as i32))
    //     .unique()
    //     .map(|(x, y, z)| ChunkCoordinates::from_voxel_coordinates(x, y, z))
    //     .take(5);
    //
    // for (cc, (x, y, z)) in coords {
    //     let chunk_op = chunks.iter_mut()
    //         .find(|(&ccc, _)| ccc == cc)
    //         .map(|(_, ch)| ch);
    //     if let Some(mut chunk) = chunk_op {
    //         unsafe {
    //             chunk.set_unchecked(x as u8, y as u8, z as u8, Some(Block{block_type: &BlockType::COPPER}));
    //             chunk.is_updated = true;
    //         }
    //     }
    // }
}

pub struct YamcPlayerPlugin;

impl Plugin for YamcPlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system_to_stage(StartupStage::PostStartup, setup_players)
            .add_system(interact_with_chunk);
    }
}