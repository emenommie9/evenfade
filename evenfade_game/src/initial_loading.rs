use std::time::Duration;

use bevy::prelude::*;

use crate::global_state::GlobalState;

#[derive(Component)]
pub struct InitialLoading;

#[derive(Resource)]
pub struct MinimumLoadTimer {
    timer: Timer,
}

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(MinimumLoadTimer {
        timer: Timer::new(Duration::from_secs(5), TimerMode::Once),
    });

    commands.spawn((Camera2d, InitialLoading));
    commands.spawn((
        Text::new("Loading..."),
        TextFont {
            font: asset_server.load("fonts/NotoSans-Regular.ttf"),
            font_size: 64.0,
            ..default()
        },
        TextShadow::default(),
        TextLayout::new_with_justify(JustifyText::Center),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(5.0),
            right: Val::Px(5.0),
            ..default()
        },
        InitialLoading,
    ));
}

pub fn despawn(mut commands: Commands, q: Query<(Entity, &InitialLoading)>) {
    for (e, _) in q.iter() {
        commands.entity(e).despawn();
    }
}

pub fn check_done_loading(
    time: Res<Time>,
    mut timer: ResMut<MinimumLoadTimer>,
    mut next_state: ResMut<NextState<GlobalState>>,
) {
    timer.timer.tick(time.delta());

    if timer.timer.finished() {
        next_state.set(GlobalState::MainMenu);
    }
}
