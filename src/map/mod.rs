use bevy::prelude::*;
use generator::*;
use chunk::Chunk;
use crate::utils::{ChunkPos, CHUNK_SIZE_F32};

pub mod generator;
pub mod render;
pub mod chunk;

const VISIBLE_CHUNKS_DISTANCE: usize = 10;
const CHUNKS_CUT_DISTANCE: usize = 0;

pub struct MapGenerationPlugin {
    pub seed: u32,
}

impl Plugin for MapGenerationPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(Generator { seed: self.seed })
            .add_systems(Update, chunk_spawner)
            .add_systems(Update, chunk_despawner);
    }
}

fn chunk_spawner(map: Res<Generator>,
                 cameras: Query<(&Transform, &Camera)>,
                 mut commands: Commands,
                 query: Query<&ChunkPos>,)
{
    for (tr, _) in &cameras {
        use std::time::Instant;
        let now = Instant::now();

        let ch_x = (tr.translation.x / (CHUNK_SIZE_F32 * CUBE_SIDE)).floor() as i32;
        let ch_z = (tr.translation.z / (CHUNK_SIZE_F32 * CUBE_SIDE)).floor() as i32;
        let chunks = get_visible_chunks(ch_x, ch_z);
        let new_chunks: Vec<_> = chunks.iter()
            .filter(|(x, z)| !query.iter().any(|c| c.0.x == *x && c.0.z == *z))
            .collect();
        if new_chunks.len() == 0 {
            return;
        }
        let &(nch_x, nch_z) = new_chunks[0];
        info!("Spawning chunk ({nch_x}, {nch_z})");
        spawn_chunk_column(nch_x, nch_z, &map, &mut commands);
        let total = now.elapsed();
        info!("Chunk ({nch_x}, {nch_z}) spawned. Total time: {total:.2?}.");
    }
}

fn get_visible_chunks(ch_x: i32, ch_z: i32) -> Vec<(i32, i32)> {
    let mut result = Vec::with_capacity(VISIBLE_CHUNKS_DISTANCE * VISIBLE_CHUNKS_DISTANCE);
    for x in (ch_x - (VISIBLE_CHUNKS_DISTANCE as i32))..=(ch_x + (VISIBLE_CHUNKS_DISTANCE as i32)) {
        for z in (ch_z - (VISIBLE_CHUNKS_DISTANCE as i32))..=(ch_z + (VISIBLE_CHUNKS_DISTANCE as i32)) {
            result.push((x, z));
        }
    }

    if CHUNKS_CUT_DISTANCE == 0 {
        return result;
    }

    for x in (ch_x - (CHUNKS_CUT_DISTANCE as i32))..=(ch_x + (CHUNKS_CUT_DISTANCE as i32)) {
        for z in (ch_z - (CHUNKS_CUT_DISTANCE as i32))..=(ch_z + (CHUNKS_CUT_DISTANCE as i32)) {
            result.retain(|&(rx, rz)| !(rx == x && rz == z))
        }
    }

    result
}

fn chunk_despawner(cameras: Query<(&Transform, &Camera)>,
                   query: Query<(Entity, &ChunkPos)>,
                   mut commands: Commands,) {
    let mut pos = (&cameras).iter()
        .map(|x| x.0.translation)
        .fold(Vec3::ZERO, |_, x| x)
        / (CHUNK_SIZE_F32 * CUBE_SIDE);
    pos = Vec3::new(pos.x.round(), pos.y.round(), pos.z.round());

    for (entity, &coords/*, handle*/) in &query {
        if coords.planar_distance(&ChunkPos::new(pos.x as i32, 0, pos.z as i32)) > VISIBLE_CHUNKS_DISTANCE as i32 * 3
            || coords.planar_distance(&ChunkPos::new(pos.x as i32, 0, pos.z as i32)) < CHUNKS_CUT_DISTANCE as i32 {
            info!("Despawning chunk ({}, {})", coords.0.x, coords.0.z);
            commands
                .entity(entity)
                .despawn();
        }
    }
}

fn spawn_chunk_column(ch_x: i32, ch_z: i32, map: &Generator, commands: &mut Commands) {
    for (ch_y, chunk) in map.get_chunk_column(ch_x, ch_z).iter().enumerate() {
        let chunk_coordinates = ChunkPos::new(ch_x, ch_y as i32, ch_z);
        commands.spawn((chunk.clone(), chunk_coordinates));
    }
}
