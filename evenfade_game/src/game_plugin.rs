use bevy::input_focus::{InputDispatchPlugin, tab_navigation::*};
use bevy::prelude::*;

use crate::global_state::GlobalState;
use crate::main_menu::MainMenuButtonClicked;
use crate::{initial_loading, main_menu};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((InputDispatchPlugin, TabNavigationPlugin));

        app.init_state::<GlobalState>();

        app.add_systems(OnEnter(GlobalState::InitialLoading), initial_loading::setup);

        app.add_systems(
            FixedUpdate,
            initial_loading::check_done_loading.run_if(in_state(GlobalState::InitialLoading)),
        );

        app.add_systems(
            OnExit(GlobalState::InitialLoading),
            initial_loading::despawn,
        );

        app.add_event::<MainMenuButtonClicked>();

        app.add_systems(OnEnter(GlobalState::MainMenu), main_menu::setup);

        app.add_systems(
            Update,
            (
                main_menu::check_focus,
                main_menu::check_interaction,
                main_menu::check_keyboard_click,
                main_menu::check_button_clicked,
            )
                .run_if(in_state(GlobalState::MainMenu)),
        );
    }
}
