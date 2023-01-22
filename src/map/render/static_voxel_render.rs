use bevy::prelude::*;
use bevy::prelude::CoreStage::PostUpdate;
use bevy::render::mesh::*;
use crate::map::chunk::Chunk;
use crate::map::chunk::chunk_coordinates::ChunkCoordinates;

pub const VOXEL_HALF_SIDE: f32 = 0.5;

const SIDES_VERTICES: [[Vec3; 4]; 6] = [
    [ // TOP
        Vec3::new(VOXEL_HALF_SIDE, VOXEL_HALF_SIDE, VOXEL_HALF_SIDE),
        Vec3::new(-VOXEL_HALF_SIDE, VOXEL_HALF_SIDE, VOXEL_HALF_SIDE),
        Vec3::new(-VOXEL_HALF_SIDE, VOXEL_HALF_SIDE, -VOXEL_HALF_SIDE),
        Vec3::new(VOXEL_HALF_SIDE, VOXEL_HALF_SIDE, -VOXEL_HALF_SIDE),
    ],
    [ // BOTTOM
        Vec3::new(VOXEL_HALF_SIDE, -VOXEL_HALF_SIDE, -VOXEL_HALF_SIDE),
        Vec3::new(-VOXEL_HALF_SIDE, -VOXEL_HALF_SIDE, -VOXEL_HALF_SIDE),
        Vec3::new(-VOXEL_HALF_SIDE, -VOXEL_HALF_SIDE, VOXEL_HALF_SIDE),
        Vec3::new(VOXEL_HALF_SIDE, -VOXEL_HALF_SIDE, VOXEL_HALF_SIDE),
    ],
    [ // RIGHT
        Vec3::new(VOXEL_HALF_SIDE, VOXEL_HALF_SIDE, -VOXEL_HALF_SIDE),
        Vec3::new(VOXEL_HALF_SIDE, -VOXEL_HALF_SIDE, -VOXEL_HALF_SIDE),
        Vec3::new(VOXEL_HALF_SIDE, -VOXEL_HALF_SIDE, VOXEL_HALF_SIDE),
        Vec3::new(VOXEL_HALF_SIDE, VOXEL_HALF_SIDE, VOXEL_HALF_SIDE),
    ],
    [ // LEFT
        Vec3::new(-VOXEL_HALF_SIDE, VOXEL_HALF_SIDE, VOXEL_HALF_SIDE),
        Vec3::new(-VOXEL_HALF_SIDE, -VOXEL_HALF_SIDE, VOXEL_HALF_SIDE),
        Vec3::new(-VOXEL_HALF_SIDE, -VOXEL_HALF_SIDE, -VOXEL_HALF_SIDE),
        Vec3::new(-VOXEL_HALF_SIDE, VOXEL_HALF_SIDE, -VOXEL_HALF_SIDE),
    ],
    [ // FORWARD
        Vec3::new(VOXEL_HALF_SIDE, -VOXEL_HALF_SIDE, VOXEL_HALF_SIDE),
        Vec3::new(-VOXEL_HALF_SIDE, -VOXEL_HALF_SIDE, VOXEL_HALF_SIDE),
        Vec3::new(-VOXEL_HALF_SIDE, VOXEL_HALF_SIDE, VOXEL_HALF_SIDE),
        Vec3::new(VOXEL_HALF_SIDE, VOXEL_HALF_SIDE, VOXEL_HALF_SIDE),
    ],
    [ // BACKWARD
        Vec3::new(VOXEL_HALF_SIDE, VOXEL_HALF_SIDE, -VOXEL_HALF_SIDE),
        Vec3::new(-VOXEL_HALF_SIDE, VOXEL_HALF_SIDE, -VOXEL_HALF_SIDE),
        Vec3::new(-VOXEL_HALF_SIDE, -VOXEL_HALF_SIDE, -VOXEL_HALF_SIDE),
        Vec3::new(VOXEL_HALF_SIDE, -VOXEL_HALF_SIDE, -VOXEL_HALF_SIDE),
    ],
];

const SIDES_INDICES: [u32; 6] = [2, 1, 0, 3, 2, 0];
const SIDES_OFFSETS: [(i32, i32, i32); 6] = [(0, 1, 0), (0, -1, 0), (1, 0, 0), (-1, 0, 0), (0, 0, 1), (0, 0, -1)];

impl From<&Chunk> for Mesh {
    fn from(chunk: &Chunk) -> Self {
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        let mut vertices: Vec<[f32; 3]> = Vec::new();
        let mut indices: Vec<u32> = Vec::new();
        let mut colors: Vec<[f32; 4]> = Vec::new();

        let mut current_index: u32 = 0;

        for ((x, y, z), block) in chunk.iter_with_pos().filter(|(_, b)| b.is_some()) {
            let sides = SIDES_OFFSETS
                .map(|(ox, oy, oz)| (x as i32 + ox, y as i32 + oy, z as i32 + oz))
                .into_iter()
                .enumerate()
                .filter(|(_, (sx, sy, sz))| !chunk.get_block_at(*sx, *sy, *sz)
                    .map(|op| op.is_some()).unwrap_or(false))
                .map(|(i, _)| i);

            for side in sides {
                vertices.extend(
                    SIDES_VERTICES[side].map(|v| [
                        v.x + (x as f32) * VOXEL_HALF_SIDE * 2.0,
                        v.y + (y as f32) * VOXEL_HALF_SIDE * 2.0,
                        v.z + (z as f32) * VOXEL_HALF_SIDE * 2.0,
                    ])
                );
                indices.extend(SIDES_INDICES.map(|si| si + (current_index * 4)));
                current_index += 1;
                colors.extend([[
                    block.unwrap().block_type.color.r(),
                    block.unwrap().block_type.color.g(),
                    block.unwrap().block_type.color.b(),
                    1.0]; 4]);
            }
        }

        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION,
                              VertexAttributeValues::Float32x3(vertices));
        mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR,
                              VertexAttributeValues::Float32x4(colors));
        mesh.set_indices(Some(Indices::U32(indices)));

        mesh
    }
}

pub struct StaticVoxelRenderPlugin;

impl Plugin for StaticVoxelRenderPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(spawn_mesh)
            .add_system_to_stage(PostUpdate, update_mesh);
    }
}

fn spawn_mesh(query: Query<(Entity, &ChunkCoordinates, &Chunk), Added<Chunk>>,
              mut commands: Commands,
              mut materials: ResMut<Assets<StandardMaterial>>,
              mut meshes: ResMut<Assets<Mesh>>,) {
    for (entity, chunk_coordinates, chunk) in &query {
        commands
            .entity(entity)
            .insert(MaterialMeshBundle {
                mesh: meshes.add(chunk.into()),
                material: materials.add(StandardMaterial {
                    unlit: true,
                    ..default()
                }),
                transform: Transform::from_translation(chunk_coordinates.global_pos()),
                global_transform: Default::default(),
                visibility: Visibility::VISIBLE,
                computed_visibility: Default::default(),
            });
    }
}

fn update_mesh(mut query: Query<(&mut Chunk, &mut Handle<Mesh>)>, mut meshes: ResMut<Assets<Mesh>>) {
    for (mut chunk, mut handle) in query.iter_mut() {
        if !chunk.is_updated {
            continue;
        }
        chunk.is_updated = false;
        let chunk: &Chunk = chunk.into_inner();
        *handle.into_inner() = meshes.add(chunk.into());
    }
}