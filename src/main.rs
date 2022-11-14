mod mapgen;

use bevy::prelude::*;
use bevy_flycam::PlayerPlugin;
use crate::mapgen::MapGenerationPlugin;

#[bevy_main]
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .add_plugin(MapGenerationPlugin)
        // .add_startup_system(setup_scene)
        .run();
}