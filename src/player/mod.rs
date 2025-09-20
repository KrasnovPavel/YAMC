use bevy::prelude::*;
use itertools::Itertools;
use crate::map::chunk::{Block, BlockType, Chunk};
use crate::utils::{ChunkPos, WorldBlockPos, WorldPos};

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
                       mut chunks: Query<(&ChunkPos, &mut Chunk)>) {
    // let transform_op = players.iter().next();
    // if transform_op.is_none() {
    //     return;
    // }
    // let transform = transform_op.unwrap();
    //
    // let coords = std::iter::repeat(transform.translation)
    //     .enumerate()
    //     .map(|(i, pos)| pos + transform.forward() * i as f32)
    //     .map (|pos| WorldPos(pos))
    //     .map(|pos| WorldBlockPos(IVec3::new(pos.0.x as i32, pos.0.y as i32, pos.0.z as i32)))
    //     .unique()
    //     .map(|pos| ChunkPos::from(pos.into()))
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
            .add_systems(PostStartup, setup_players)
            .add_systems(Update, interact_with_chunk);
    }
}