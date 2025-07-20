use bevy::asset::RenderAssetUsages;
use bevy::log::info;
use bevy::math::Vec3;
use bevy::prelude::Mesh;
use bevy::render::mesh::{Indices, PrimitiveTopology, VertexAttributeValues};
use crate::map::chunk::Chunk;

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

pub trait CulledChunkMesher {
    fn create_mesh_culled(&self) -> Mesh;
}

impl CulledChunkMesher for Chunk {
    fn create_mesh_culled(&self) -> Mesh {
        use std::time::Instant;
        let now = Instant::now();
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD);
        let mut vertices: Vec<[f32; 3]> = Vec::new();
        let mut indices: Vec<u32> = Vec::new();
        let mut colors: Vec<[f32; 4]> = Vec::new();

        let mut current_index: u32 = 0;

        for ((x, y, z), block) in self.iter_with_pos().filter(|(_, b)| b.is_some()) {
            let sides = SIDES_OFFSETS
                .map(|(ox, oy, oz)| (x as i32 + ox, y as i32 + oy, z as i32 + oz))
                .into_iter()
                .enumerate()
                .filter(|(_, (sx, sy, sz))| !self.get_block_at(*sx, *sy, *sz)
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
                    block.unwrap().block_type.color.to_srgba().red,
                    block.unwrap().block_type.color.to_srgba().green,
                    block.unwrap().block_type.color.to_srgba().blue,
                    1.0]; 4]);
            }
        }

        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION,
                              VertexAttributeValues::Float32x3(vertices));
        mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR,
                              VertexAttributeValues::Float32x4(colors));
        mesh.insert_indices(Indices::U32(indices));

        let total = now.elapsed();
        info!("Chunk meshed. Total time: {total:.2?}.");

        mesh
    }
}