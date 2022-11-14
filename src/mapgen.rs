use noise::{
    core::perlin::perlin_2d,
    permutationtable::PermutationTable,
    utils::{NoiseMap, PlaneMapBuilder, NoiseMapBuilder},
};

use bevy::prelude::*;
use bevy_aabb_instancing::{ColorOptions, ColorOptionsMap, Cuboid, Cuboids, ScalarHueColorOptions, VertexPullingRenderPlugin, COLOR_MODE_RGB};

pub struct MapGenerationPlugin;

impl Plugin for MapGenerationPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Msaa { samples: 1 })
            .add_plugin(VertexPullingRenderPlugin { outlines: true })
            .add_startup_system(spawn_map);
    }
}

fn run_mapgen(seed: u32) -> NoiseMap
{
    let hasher = PermutationTable::new(seed);
    PlaneMapBuilder::new_fn(perlin_2d, &hasher)
        .set_size(128, 128)
        .set_x_bounds(-5.0, 5.0)
        .set_y_bounds(-5.0, 5.0)
        .build()
}

fn spawn_map(mut commands: Commands, mut color_options_map: ResMut<ColorOptionsMap>)
{
    let map = run_mapgen(1337);
    let color_options_id = color_options_map.push(ColorOptions {
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

    let cube_size = Vec3::new(0.5, 0.5, 0.5);
    let mut instances = Vec::with_capacity(128 * 128 * 21);
    for x in 0..128 {
        for z in 0..128 {
            let max_y = (map.get_value(x, z) * 10.0).floor() as i64;
            for y in -11..max_y {
                let c = Vec3::new(x as f32, y as f32, z as f32);
                let min = c - cube_size;
                let max = c + cube_size;
                let color = match y {
                    -11..=0 => Color::BLUE,
                    1..=5 => Color::DARK_GREEN,
                    6..=10 => Color::WHITE,
                    _ => Color::BLACK,
                };

                instances.push(Cuboid::new(min, max, color.as_rgba_u32(), true, 0));
            }
        }
    }

    let cuboids = Cuboids::new(instances);
    let aabb = cuboids.aabb();
    commands
        .spawn(SpatialBundle::default())
        .insert((cuboids, aabb, color_options_id));
}