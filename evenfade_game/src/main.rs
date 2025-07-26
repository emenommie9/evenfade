use bevy::{log::LogPlugin, prelude::*};

use crate::game_plugin::GamePlugin;

mod game_plugin;

/// Main entry point to the game, setup the bevy app.
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(LogPlugin {
            filter: "info,wgpu_core=warn,wgpu_hal=warn,naga=warn,evenfade_game=debug".into(),
            level: bevy::log::Level::DEBUG,
            custom_layer: |_| None,
        }))
        .add_plugins(GamePlugin)
        .run();
}
