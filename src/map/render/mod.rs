pub mod chunk_coordinates;

use bevy::prelude::*;
use bevy_aabb_instancing::{ColorOptions, ColorOptionsMap, Cuboid, Cuboids, ScalarHueColorOptions, VertexPullingRenderPlugin, COLOR_MODE_RGB, ColorOptionsId};
use crate::map::generator::*;
use crate::map::render::chunk_coordinates::ChunkCoordinates;

const DRAW_CHUNK_SIZE: usize = 1024;
const CUBE_SIZE: Vec3 = Vec3::new(0.5, 0.5, 0.5);
const VISIBLE_CHUNKS_DISTANCE: usize = 10 * DEBUG_WORLD_SCALE;

pub struct MapGenerationPlugin {
    pub(crate) seed: u32,
}

#[derive(Resource)]
struct RendererMetadata {
    color_options_id: ColorOptionsId,
    cuboids_buffer: Vec<Cuboid>,
}

impl Default for RendererMetadata {
    fn default() -> Self {
        RendererMetadata {
            color_options_id: ColorOptionsId(0),
            cuboids_buffer: Vec::new(),
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
                 mut metadata: ResMut<RendererMetadata>,
                 mut commands: Commands,
                 query: Query<&ChunkCoordinates>)
{
    for (tr, _) in &cameras {
        let ch_x = (tr.translation.x / (CHUNK_RESOLUTION as f32 * CUBE_SIDE)).floor() as i64;
        let ch_z = (tr.translation.z / (CHUNK_RESOLUTION as f32 * CUBE_SIDE)).floor() as i64;
        let chunks = get_visible_chunks(ch_x, ch_z);
        let new_chunks: Vec<_> = chunks.iter()
            .filter(|(x, z)| !query.iter().any(|c| c.x == *x && c.z == *z))
            .collect();
        if new_chunks.len() == 0 {
            return;
        }
        let &(nch_x, nch_z) = new_chunks[0];
        spawn_chunk(map.get_chunk(nch_x, nch_z), &mut metadata, &mut commands);
        info!("Spawning chunk ({nch_x}, {nch_z})");
    }
}

fn get_visible_chunks(ch_x: i64, ch_z: i64) -> Vec<(i64, i64)> {
    let mut result = Vec::with_capacity(VISIBLE_CHUNKS_DISTANCE * VISIBLE_CHUNKS_DISTANCE);
    for x in (ch_x - (VISIBLE_CHUNKS_DISTANCE as i64))..=(ch_x + (VISIBLE_CHUNKS_DISTANCE as i64)) {
        for z in (ch_z - (VISIBLE_CHUNKS_DISTANCE as i64))..=(ch_z + (VISIBLE_CHUNKS_DISTANCE as i64)) {
            result.push((x, z));
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
        if pos.distance(Vec3::from(coords)) > VISIBLE_CHUNKS_DISTANCE as f32 * 2.0 {
            // info!("Despawning chunk ({}, {}, {})", coords.x, coords.y, coords.z);
            commands
                .entity(entity)
                .despawn();
        }
    }
}

fn spawn_chunk(chunk: Chunk, metadata: &mut ResMut<RendererMetadata>, mut commands: &mut Commands) {
    let chunk_coordinates = ChunkCoordinates::new(chunk.x, chunk.y);
    let mut i = 0;
    for z in 0..chunk.cubes.len() {
        for x in 0..chunk.cubes[z].len() {
            for (y, block) in chunk.cubes[z][x].iter() {
                add_cube(x as i64, *y, z as i64, &mut i, block.color, &chunk_coordinates, metadata, commands);
            }
        }
    }
    spawn_cuboid(chunk_coordinates.clone(), &metadata.cuboids_buffer[0..i].to_owned(), &mut commands, &metadata.color_options_id)
}

fn add_cube(x: i64, y: i8, z: i64, i: &mut usize, color: Color,
            chunk_coordinates: &ChunkCoordinates,
            metadata: &mut ResMut<RendererMetadata>,
            mut commands: &mut Commands) {
    let c = Vec3::new(x as f32, y as f32, z as f32) + Vec3::from(*chunk_coordinates) * (CHUNK_RESOLUTION as f32);
    let min = (c - CUBE_SIZE) * CUBE_SIDE;
    let max = (c + CUBE_SIZE) * CUBE_SIDE;
    if *i == DRAW_CHUNK_SIZE {
        spawn_cuboid(chunk_coordinates.clone(), &metadata.cuboids_buffer, &mut commands, &metadata.color_options_id);
        *i = 0;
    }
    metadata.cuboids_buffer[*i] = Cuboid::new(min, max, color.as_rgba_u32(), true, 0);
    *i += 1;
}

fn spawn_cuboid(chunk_coordinates: ChunkCoordinates,
                instances: &Vec<Cuboid>,
                commands: &mut Commands,
                color_options_id: &ColorOptionsId)
{
    let cuboids = Cuboids::new(instances.clone());
    let aabb = cuboids.aabb();
    commands
        .spawn(SpatialBundle::default())
        .insert((cuboids, aabb, *color_options_id, chunk_coordinates));
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

    metadata.cuboids_buffer = Vec::with_capacity(DRAW_CHUNK_SIZE);
    metadata.cuboids_buffer.resize(DRAW_CHUNK_SIZE, Cuboid::new(Vec3::ZERO, Vec3::ZERO, 0, false, 0));
}