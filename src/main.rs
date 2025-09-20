mod map;
mod player;
mod utils;

use bevy::prelude::*;
use bevy_fly_camera::{FlyCamera, FlyCameraPlugin};
use crate::map::MapGenerationPlugin;
use crate::map::render::StaticVoxelRenderPlugin;
use std::env;
use crate::player::YamcPlayerPlugin;

fn setup_camera(mut commands: Commands) {
    commands
        .spawn(Camera3d::default())
        .insert(FlyCamera::default());
}

#[bevy_main]
fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    App::new()
        .insert_resource(ClearColor(Color::srgb_u8(0,0,255)))
        .add_plugins(DefaultPlugins)
        // .add_plugins(LogDiagnosticsPlugin::default())
        // .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_systems(Startup, setup_camera)
        .add_plugins(FlyCameraPlugin)
        .add_plugins(YamcPlayerPlugin)
        .add_plugins(MapGenerationPlugin {seed: 1337} )
        .add_plugins(StaticVoxelRenderPlugin)
        .run();
}