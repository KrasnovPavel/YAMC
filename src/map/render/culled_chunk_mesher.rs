use std::collections::HashMap;
use bevy::asset::RenderAssetUsages;
use bevy::log::info;
use bevy::math::{IVec3, Vec3};
use bevy::prelude::Mesh;
use bevy::render::mesh::{Indices, PrimitiveTopology, VertexAttributeValues};
use crate::map::chunk::Chunk;
use crate::utils::{BlockPos, ChunkPos, WorldBlockPos};

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
    fn create_mesh_culled(&self, pos: &ChunkPos, map: &HashMap<ChunkPos, &Chunk>) -> Mesh;
    fn update_mesh_culled(&self, mesh: &mut Mesh, pos: &ChunkPos, map: &HashMap<ChunkPos, &Chunk>);
}

impl CulledChunkMesher for Chunk {
    fn create_mesh_culled(&self, pos: &ChunkPos, map: &HashMap<ChunkPos, &Chunk>) -> Mesh {
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD);
        self.update_mesh_culled(&mut mesh, pos, map);
        mesh
    }

    fn update_mesh_culled(&self, mesh: &mut Mesh, chunk_pos: &ChunkPos, map: &HashMap<ChunkPos, &Chunk>) {
        use std::time::Instant;
        let now = Instant::now();
        let mut vertices: Vec<[f32; 3]> = Vec::new();
        let mut indices: Vec<u32> = Vec::new();
        let mut colors: Vec<[f32; 4]> = Vec::new();

        let mut current_index: u32 = 0;

        let mut blocks_counter = 0;

        for (block_pos, block) in self.iter_with_pos().filter(|(_, b)| b.is_some()) {
            let starting_time = now.elapsed();
            blocks_counter += 1;
            let sides = SIDES_OFFSETS
                .map(|(ox, oy, oz)| {
                    let side_pos = BlockPos::new(block_pos.0.x + ox, block_pos.0.y + oy, block_pos.0.z + oz);
                    WorldBlockPos::from(chunk_pos, &side_pos)
                })
                .into_iter()
                .enumerate()
                .filter(|(_, side_pos)| !has_in_map_at(side_pos, chunk_pos, self, map))
                .map(|(i, _)| i);

            for side in sides {
                vertices.extend(
                    SIDES_VERTICES[side].map(|v| [
                        v.x + (block_pos.0.x as f32) * VOXEL_HALF_SIDE * 2.0,
                        v.y + (block_pos.0.y as f32) * VOXEL_HALF_SIDE * 2.0,
                        v.z + (block_pos.0.z as f32) * VOXEL_HALF_SIDE * 2.0,
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

        mesh.remove_attribute(Mesh::ATTRIBUTE_POSITION);
        mesh.remove_attribute(Mesh::ATTRIBUTE_COLOR);
        mesh.remove_indices();

        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION,
                              VertexAttributeValues::Float32x3(vertices));
        mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR,
                              VertexAttributeValues::Float32x4(colors));
        mesh.insert_indices(Indices::U32(indices));

        let total = now.elapsed();
        if (blocks_counter > 0) {
            info!("Chunk meshed ({blocks_counter} blocks, {current_index} sides). Total time: {total:.0?}.");
        }
    }
}

fn has_in_map_at(pos: &WorldBlockPos,
                 current_chunk_pos: &ChunkPos,
                 current_chunk: &Chunk,
                 map: &HashMap<ChunkPos, &Chunk>,) -> bool {
    let chunk_pos = pos.clone().into();
    let block_pos = pos.clone().into();
    if (chunk_pos == *current_chunk_pos) {
        return matches!(current_chunk.get_block_at(&block_pos), Ok(Some(_)));
    }

    let chunk_op = map.get(&chunk_pos);
    if chunk_op.is_none() {
        return false;
    }

    let chunk = chunk_op.unwrap();
    matches!(chunk.get_block_at(&block_pos), Ok(Some(_)))
}