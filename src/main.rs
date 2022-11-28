#![feature(exclusive_range_pattern)]

mod map;

use bevy::prelude::*;
use bevy_flycam::PlayerPlugin;
use crate::map::render::MapGenerationPlugin;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

#[bevy_main]
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // .add_plugin(LogDiagnosticsPlugin::default())
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(PlayerPlugin)
        .add_plugin(MapGenerationPlugin {seed: 1337} )
        .run();
}