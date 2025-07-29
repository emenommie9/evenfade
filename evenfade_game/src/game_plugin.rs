use bevy::prelude::*;

use crate::global_state::GlobalState;
use crate::initial_loading;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
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
    }
}
