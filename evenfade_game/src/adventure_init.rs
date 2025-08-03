use std::time::Duration;

use bevy::{asset::AssetPath, prelude::*};

use crate::global_state::GlobalState;

#[derive(Resource)]
pub enum AdventureInitState {
    PreOrPostInit,
    Init(AdventureInitParams),
}

pub struct AdventureInitParams {
    pub adventure_to_init: AssetPath<'static>,
}

#[derive(Resource)]
pub struct MinimumInitTimer(Timer);

#[derive(Component)]
pub struct AdventureInit;

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    init_state: Res<AdventureInitState>,
) {
    let AdventureInitState::Init(ref params) = *init_state else {
        panic!("Adventure init system active without correct init state")
    };

    commands.insert_resource(MinimumInitTimer(Timer::new(
        Duration::from_secs(1),
        TimerMode::Once,
    )));

    commands.spawn((Camera2d, AdventureInit));
    commands
        .spawn((
            Node {
                display: Display::Flex,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            AdventureInit,
        ))
        .with_child((
            Text::new(format!("Loading adventure {}...", params.adventure_to_init)),
            TextFont {
                font: asset_server.load("fonts/NotoSans-Regular.ttf"),
                font_size: 24.0,
                ..default()
            },
            TextLayout::new_with_justify(JustifyText::Center),
        ));
}

pub fn check_done_loading(
    time: Res<Time>,
    mut timer: ResMut<MinimumInitTimer>,
    mut next_state: ResMut<NextState<GlobalState>>,
) {
    timer.0.tick(time.delta());

    if timer.0.finished() {
        next_state.set(GlobalState::AdventurePlaying);
    }
}

pub fn despawn(mut commands: Commands, query: Query<Entity, With<AdventureInit>>) {
    for e in query.iter() {
        commands.entity(e).despawn();
    }
}
