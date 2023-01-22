#![feature(exclusive_range_pattern)]

mod map;
mod player;

use bevy::prelude::*;
use bevy_flycam::PlayerPlugin;
use crate::map::MapGenerationPlugin;
use crate::map::render::StaticVoxelRenderPlugin;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use std::env;
use crate::player::YamcPlayerPlugin;

#[bevy_main]
fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    App::new()
        .insert_resource(ClearColor(Color::BLUE))
        .add_plugins(DefaultPlugins)
        // .add_plugin(LogDiagnosticsPlugin::default())
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(PlayerPlugin)
        .add_plugin(YamcPlayerPlugin)
        .add_plugin(MapGenerationPlugin {seed: 1337} )
        .add_plugin(StaticVoxelRenderPlugin)
        .run();
}