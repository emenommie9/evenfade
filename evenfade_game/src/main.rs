use bevy::{log::LogPlugin, prelude::*};

use crate::game_plugin::GamePlugin;

mod game_plugin;
mod global_state;
mod initial_loading;
mod main_menu;

/// Main entry point to the game, setup the bevy app.
fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(LogPlugin {
                    filter: "info,wgpu=warn,naga=warn,evenfade_game=debug".into(),
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Evenfade".to_string(),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins(GamePlugin)
        .run();
}
