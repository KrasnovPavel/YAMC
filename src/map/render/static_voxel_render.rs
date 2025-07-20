use bevy::asset::RenderAssetUsages;
use bevy::prelude::*;
use bevy::render::mesh::*;
use crate::map::chunk::{Block, Chunk};
use crate::map::chunk::chunk_coordinates::ChunkCoordinates;
use crate::map::render::culled_chunk_mesher::CulledChunkMesher;

pub struct StaticVoxelRenderPlugin;

#[derive(Resource)]
struct DebugMeshVisualization(bool);

impl Plugin for StaticVoxelRenderPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(DebugMeshVisualization(false))
            .add_systems(Update, spawn_mesh)
            .add_systems(PostUpdate, update_mesh)
            .add_systems(Update, debug_draw_mesh)
            .add_systems(Update, toggle_debug_visualization);
    }
}

fn spawn_mesh(query: Query<(Entity, &ChunkCoordinates, &Chunk), Added<Chunk>>,
              mut commands: Commands,
              mut materials: ResMut<Assets<StandardMaterial>>,
              mut meshes: ResMut<Assets<Mesh>>,) {
    for (entity, chunk_coordinates, chunk) in &query {
        commands
            .entity(entity)
            .insert(Mesh3d(meshes.add(chunk.create_mesh_culled())))
            .insert(MeshMaterial3d(materials.add(StandardMaterial {
                unlit: true,
                ..default()
            })))
            .insert(Transform::from_translation(chunk_coordinates.global_pos()))
            .insert(Visibility::Visible);
    }
}

fn update_mesh(mut query: Query<(&mut Chunk, &mut Mesh3d)>, mut meshes: ResMut<Assets<Mesh>>) {
    for (mut chunk, mut handle) in query.iter_mut() {
        if !chunk.is_updated {
            continue;
        }
        chunk.is_updated = false;
        let chunk: &Chunk = chunk.into_inner();
        *handle.into_inner() = Mesh3d(meshes.add(chunk.create_mesh_culled()));
    }
}
fn debug_draw_mesh(
    mesh_query: Query<(&Transform, &Mesh3d)>,
    meshes: Res<Assets<Mesh>>,
    debug_vis: Res<DebugMeshVisualization>,
    mut gizmos: Gizmos,
) {
    if !debug_vis.0 {
        return;
    }
    for (transform, mesh_handle) in mesh_query.iter() {
        if let Some(mesh) = meshes.get(&mesh_handle.0) {
            if let Some(VertexAttributeValues::Float32x3(vertices)) = mesh.attribute(Mesh::ATTRIBUTE_POSITION) {
                if let Some(Indices::U32(indices)) = mesh.indices() {
                    // Draw each triangle
                    for triangle in indices.chunks(3) {
                        let world_vertices: Vec<Vec3> = triangle
                            .iter()
                            .map(|&i| {
                                let vertex = Vec3::from(vertices[i as usize]);
                                transform.transform_point(vertex)
                            })
                            .collect();

                        if world_vertices.len() == 3 {
                            // Draw triangle edges
                            gizmos.line(world_vertices[0], world_vertices[1], bevy::color::palettes::css::RED);
                            gizmos.line(world_vertices[1], world_vertices[2], bevy::color::palettes::css::RED);
                            gizmos.line(world_vertices[2], world_vertices[0], bevy::color::palettes::css::RED);
                        }
                    }
                }
            }
        }
    }
}

fn toggle_debug_visualization(mut debug_vis: ResMut<DebugMeshVisualization>,
	time: Res<Time>,
	keyboard_input: Res<ButtonInput<KeyCode>>,) {
    if keyboard_input.just_pressed(KeyCode::F3) {  // or any other key you prefer
        debug_vis.0 = !debug_vis.0;
    }
}
