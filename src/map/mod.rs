use bevy::prelude::*;
use generator::*;
use chunk::Chunk;
use chunk::chunk_coordinates::ChunkCoordinates;

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
                 query: Query<&ChunkCoordinates>,)
{
    for (tr, _) in &cameras {
        use std::time::Instant;
        let now = Instant::now();

        let ch_x = (tr.translation.x / (Chunk::LENGTH as f32 * CUBE_SIDE)).floor() as i32;
        let ch_z = (tr.translation.z / (Chunk::WIDTH as f32 * CUBE_SIDE)).floor() as i32;
        let chunks = get_visible_chunks(ch_x, ch_z);
        let new_chunks: Vec<_> = chunks.iter()
            .filter(|(x, z)| !query.iter().any(|c| c.x == *x && c.z == *z))
            .collect();
        if new_chunks.len() == 0 {
            return;
        }
        let &(nch_x, nch_z) = new_chunks[0];
        info!("Spawning chunk ({nch_x}, {nch_z})");
        spawn_chunk(nch_x, nch_z, &map, &mut commands);
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
                   query: Query<(Entity, &ChunkCoordinates)>,
                   mut commands: Commands,) {
    let mut pos = (&cameras).iter()
        .map(|x| x.0.translation)
        .fold(Vec3::ZERO, |_, x| x)
        / (Chunk::LENGTH as f32 * CUBE_SIDE);
    pos = Vec3::new(pos.x.round(), pos.y.round(), pos.z.round());

    for (entity, &coords/*, handle*/) in &query {
        if coords.distance(pos.x as i32, pos.z as i32) > VISIBLE_CHUNKS_DISTANCE as i32 * 3
            || coords.distance(pos.x as i32, pos.z as i32) < CHUNKS_CUT_DISTANCE as i32 {
            info!("Despawning chunk ({}, {})", coords.x, coords.z);
            commands
                .entity(entity)
                .despawn();
        }
    }
}

fn spawn_chunk(ch_x: i32, ch_z: i32, map: &Generator, commands: &mut Commands) {
    let chunk_coordinates = ChunkCoordinates::new(ch_x, ch_z);
    let chunk = map.get_chunk(ch_x, ch_z);
    commands.spawn((chunk, chunk_coordinates));
}
