use std::collections::{HashMap, HashSet};
use bevy::prelude::*;
use crate::map::chunk::{Block, Chunk};
use crate::map::chunk::chunk_coordinates::ChunkCoordinates;
use crate::map::render::culled_chunk_mesher::CulledChunkMesher;

pub struct StaticVoxelRenderPlugin;

impl Plugin for StaticVoxelRenderPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(PostUpdate, (spawn_mesh, remesh_neighbour_chunks_after_spawn));
    }
}

const SIDES_OFFSETS: [(i32, i32, i32); 4] = [(1, 0, 0), (-1, 0, 0), (0, 0, 1), (0, 0, -1)];

fn spawn_mesh(query: Query<(Entity, &ChunkCoordinates, &Chunk), Added<Chunk>>,
              mut commands: Commands,
              mut materials: ResMut<Assets<StandardMaterial>>,
              mut meshes: ResMut<Assets<Mesh>>,) {
    let map = HashMap::from_iter(query.iter()
        .map(|(_, chunk_coordinates, chunk)| (*chunk_coordinates, chunk)));
    let mut counter = 0;
    for (entity, chunk_coordinates, chunk) in &query {
        commands
            .entity(entity)
            .insert(Mesh3d(meshes.add(chunk.create_mesh_culled(chunk_coordinates, &map))))
            .insert(MeshMaterial3d(materials.add(StandardMaterial {
                unlit: true,
                ..default()
            })))
            .insert(Transform::from_translation(chunk_coordinates.global_pos()))
            .insert(Visibility::Visible);
        counter += 1;
    }
    if counter > 0 {
        info!("Spawned and meshed {counter} chunks");
    }
}

fn remesh_neighbour_chunks_after_spawn(fresh_chunks: Query<&ChunkCoordinates, Added<Chunk>>,
                                       query: Query<(&ChunkCoordinates, &Chunk, &Mesh3d)>,
                                       mut meshes: ResMut<Assets<Mesh>>) {
    if (fresh_chunks.is_empty()) {
        return;
    }

    let mut chunks_to_update = HashSet::new();
    for pos in fresh_chunks {
        for (x, y, z) in SIDES_OFFSETS {
            let neighbour_pos = ChunkCoordinates::new(pos.x + x, pos.y + y, pos.z + z);
            chunks_to_update.insert(neighbour_pos);
        }
    }

    let fresh_chunks_len = fresh_chunks.iter().count();
    let chunks_to_update_len = chunks_to_update.len();
    info!("Spawned {fresh_chunks_len} chunks, remeshing up to {chunks_to_update_len} chunks");

    let map = HashMap::from_iter(query.iter()
        .map(|(chunk_coordinates, chunk, _)| (*chunk_coordinates, chunk)));

    let mut counter = 0;

    for (pos, chunk, handle) in query.iter() {
        if (chunks_to_update.contains(pos)) {
            let mesh = meshes.get_mut(handle.id()).unwrap();
            chunk.update_mesh_culled(mesh, pos, &map);
            counter += 1;
        }
    }
    info!("Remeshed {counter} chunks");
}

//
// fn update_mesh(mut query: Query<(&ChunkCoordinates, &mut Chunk, &Mesh3d)>, mut meshes: ResMut<Assets<Mesh>>) {
//     let mut updated_chunks = vec![];
//     for (pos, mut chunk, _) in query.iter_mut() {
//         if chunk.is_updated {
//             chunk.is_updated = false;
//             updated_chunks.push(*pos);
//         }
//     }
//
//     if (updated_chunks.is_empty()){
//         return;
//     }
//
//     let map = query.iter()
//         .map(|(chunk_coordinates, chunk, _)| (chunk_coordinates, chunk))
//         .collect::<Vec<_>>();
//
//     for (pos, chunk, handle) in query.iter() {
//         if (updated_chunks.contains(pos)) {
//             let mesh = meshes.get_mut(&handle.0).unwrap();
//             chunk.update_mesh_culled_interchunk(mesh, pos, &map);
//         }
//     }
// }
