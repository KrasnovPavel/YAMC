pub mod chunk_coordinates;

use bevy::prelude::*;
use bevy_aabb_instancing::{ColorOptions, ColorOptionsMap, Cuboid, Cuboids, ScalarHueColorOptions, VertexPullingRenderPlugin, COLOR_MODE_RGB, ColorOptionsId, CuboidsBundle};
use crate::map::generator::*;
use crate::map::render::chunk_coordinates::ChunkCoordinates;
use enumflags2::{bitflags, make_bitflags, BitFlags};
use crate::map::blocks::{BlockType, BlockKind};

const DRAW_CHUNK_SIZE: usize = 1024;
const CUBE_SIZE: Vec3 = Vec3::new(0.5, 0.5, 0.5);
const VISIBLE_CHUNKS_DISTANCE: usize = 10 * DEBUG_WORLD_SCALE;
const CHUNKS_CUT_DISTANCE: usize = 0 * DEBUG_WORLD_SCALE;

#[bitflags(default = Stone | Topping | Resources | Water)]
#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq)]
enum BlockFilter {
    Stone = 0b0001,
    Topping = 0b0010,
    Resources = 0b0100,
    Water = 0b1000,
}

pub struct MapGenerationPlugin {
    pub(crate) seed: u32,
}

#[derive(Resource)]
struct RendererMetadata {
    color_options_id: ColorOptionsId,
    block_filter: BitFlags<BlockFilter>,
}

impl Default for RendererMetadata {
    fn default() -> Self {
        RendererMetadata {
            color_options_id: ColorOptionsId(0),
            block_filter: BitFlags::default(),
            // block_filter: BlockFilter::Resources | BlockFilter::Topping,
        }
    }
}

impl Plugin for MapGenerationPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Msaa { samples: 1 })
            .add_plugin(VertexPullingRenderPlugin { outlines: true })
            .insert_resource(Generator { seed: self.seed })
            .insert_resource(RendererMetadata::default())
            .add_system(chunk_spawner)
            .add_system(chunk_despawner)
            .add_startup_system(setup);
    }
}

fn chunk_spawner(map: Res<Generator>,
                 cameras: Query<(&Transform, &Camera)>,
                 metadata: Res<RendererMetadata>,
                 mut commands: Commands,
                 query: Query<&ChunkCoordinates>)
{
    for (tr, _) in &cameras {
        use std::time::Instant;
        let now = Instant::now();

        let ch_x = (tr.translation.x / (CHUNK_RESOLUTION as f32 * CUBE_SIDE)).floor() as i32;
        let ch_z = (tr.translation.z / (CHUNK_RESOLUTION as f32 * CUBE_SIDE)).floor() as i32;
        let chunks = get_visible_chunks(ch_x, ch_z);
        let new_chunks: Vec<_> = chunks.iter()
            .filter(|(x, z)| !query.iter().any(|c| c.x == *x && c.z == *z))
            .collect();
        if new_chunks.len() == 0 {
            return;
        }
        let &(nch_x, nch_z) = new_chunks[0];
        info!("Spawning chunk ({nch_x}, {nch_z})");
        let chunk = map.get_chunk(nch_x, nch_z);
        let after_generation = now.elapsed();
        spawn_chunk(chunk, &metadata, &mut commands);
        let total = now.elapsed();
        let render_time = total - after_generation;
        info!("Chunk ({nch_x}, {nch_z}) spawned. Render time: {render_time:.2?}. Total time: {total:.2?}.");
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
                   mut commands: Commands) {
    let mut pos = (&cameras).iter()
        .map(|x| x.0.translation)
        .fold(Vec3::ZERO, |_, x| x)
        / (CHUNK_RESOLUTION as f32 * CUBE_SIDE);
    pos = Vec3::new(pos.x.round(), pos.y.round(), pos.z.round());

    for (entity, &coords) in &query {
        if coords.distance(pos.x as i32, pos.z as i32) > VISIBLE_CHUNKS_DISTANCE as i32 * 3
           || coords.distance(pos.x as i32, pos.z as i32) < CHUNKS_CUT_DISTANCE as i32 {
            info!("Despawning chunk ({}, {})", coords.x, coords.z);
            commands
                .entity(entity)
                .despawn();
        }
    }
}

fn spawn_chunk(chunk: Chunk, metadata: &Res<RendererMetadata>, mut commands: &mut Commands) {
    let chunk_coordinates = ChunkCoordinates::new(chunk.x, chunk.y);
    let mut cuboids = Vec::with_capacity(chunk.get_amount_of_cubes());
    for x in 0..CHUNK_RESOLUTION {
        for y in 0..256 {
            for z in 0..CHUNK_RESOLUTION {
                if let Some(block) = chunk.get(x, y, z) {
                    if !should_render_block(&block, metadata.block_filter) {
                        continue;
                    }

                    let cuboid = create_cuboid(x as i32, y as i32 - 127, z as i32, block.color, &chunk_coordinates);
                    cuboids.push(cuboid);
                }
            }
        }
    }

    spawn_cuboids(chunk_coordinates.clone(), cuboids, &mut commands, &metadata.color_options_id);
}

fn should_render_block(block: &BlockType, filter: BitFlags<BlockFilter>) -> bool {
    if filter.contains(BlockFilter::Water) && BlockKind::FLUID.contains(block) {
        return true;
    }
    if filter.contains(BlockFilter::Topping) && BlockKind::TOPPING.contains(block) {
        return true;
    }
    if filter.contains(BlockFilter::Stone) && BlockKind::CRUST.contains(block) {
        return true;
    }
    if filter.contains(BlockFilter::Resources) && BlockKind::RESOURCES.contains(block) {
        return true;
    }
    false
}

fn create_cuboid(x: i32, y: i32, z: i32, color: Color, chunk_coordinates: &ChunkCoordinates) -> Cuboid {
    let c = Vec3::new(x as f32, y as f32, z as f32) + Vec3::from(*chunk_coordinates) * (CHUNK_RESOLUTION as f32);
    let min = (c - CUBE_SIZE) * CUBE_SIDE;
    let max = (c + CUBE_SIZE) * CUBE_SIDE;
    Cuboid::new(min, max, color.as_rgba_u32(), true, 0)
}

fn spawn_cuboids(chunk_coordinates: ChunkCoordinates, instances: Vec<Cuboid>,
                 commands: &mut Commands, color_options_id: &ColorOptionsId)
{
    let cuboids = Cuboids::new(instances.clone());
    commands.spawn(CuboidsBundle {
        color_options_id: *color_options_id,
        spatial: SpatialBundle::default(),
        cuboids,
    }).insert(chunk_coordinates);
}

fn setup(mut color_options_map: ResMut<ColorOptionsMap>, mut metadata: ResMut<RendererMetadata>)
{
    metadata.color_options_id = color_options_map.push(ColorOptions {
        scalar_hue: ScalarHueColorOptions {
            min_visible: 0.0,
            max_visible: 1000.0,
            clamp_min: 0.0,
            clamp_max: 1000.0,
            hue_zero: 240.0,
            hue_slope: -300.0,
        },
        color_mode: COLOR_MODE_RGB,
        wireframe: 0,
    });
}