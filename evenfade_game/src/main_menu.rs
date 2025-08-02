use bevy::input_focus::{InputFocus, tab_navigation::*};
use bevy::prelude::*;

#[derive(Component)]
pub struct MainMenu;

#[derive(Copy, Clone)]
pub enum MainMenuButtonAction {
    NewGame,
    LoadGame,
    JoinGame,
    Options,
    Exit,
}

#[derive(Component)]
pub struct MainMenuButton(MainMenuButtonAction);

#[derive(Event)]
pub struct MainMenuButtonClicked(MainMenuButtonAction);

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((Camera2d, MainMenu));
    commands
        .spawn((
            Node {
                height: Val::Percent(100.0),
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Start,
                justify_content: JustifyContent::End,
                padding: UiRect {
                    left: Val::Px(20.0),
                    bottom: Val::Px(20.0),
                    ..default()
                },
                ..default()
            },
            TabGroup::new(0),
            MainMenu,
        ))
        .with_children(|parent| {
            spawn_button(
                "New game",
                MainMenuButtonAction::NewGame,
                parent,
                &asset_server,
            );
            spawn_button(
                "Load game",
                MainMenuButtonAction::LoadGame,
                parent,
                &asset_server,
            );
            spawn_button(
                "Join game",
                MainMenuButtonAction::JoinGame,
                parent,
                &asset_server,
            );
            spawn_button(
                "Options",
                MainMenuButtonAction::Options,
                parent,
                &asset_server,
            );
            spawn_button("Exit", MainMenuButtonAction::Exit, parent, &asset_server);
        });
}

fn spawn_button(
    label: &str,
    action: MainMenuButtonAction,
    parent: &mut ChildSpawnerCommands<'_>,
    asset_server: &Res<AssetServer>,
) {
    parent
        .spawn((
            Button,
            MainMenuButton(action),
            Node {
                align_content: AlignContent::Start,
                justify_content: JustifyContent::Start,
                margin: UiRect {
                    top: Val::Px(4.0),
                    bottom: Val::Px(4.0),
                    ..default()
                },
                ..default()
            },
            TabIndex(0),
        ))
        .observe(
            |mut trigger: Trigger<Pointer<Click>>, mut focus: ResMut<InputFocus>| {
                focus.0 = Some(trigger.target());
                trigger.propagate(false);
            },
        )
        .with_child((
            Text::new(label),
            TextFont {
                font: asset_server.load("fonts/NotoSans-Regular.ttf"),
                font_size: 24.0,
                ..default()
            },
        ));
}

pub fn check_focus(
    mut commands: Commands,
    focus: Res<InputFocus>,
    mut query: Query<Entity, (With<Node>, With<TabIndex>)>,
) {
    if focus.is_changed() {
        for element in query.iter_mut() {
            if focus.0 == Some(element) {
                commands.entity(element).insert(Outline {
                    color: Color::WHITE,
                    width: Val::Px(1.0),
                    offset: Val::Px(1.0),
                });
            } else {
                commands.entity(element).remove::<Outline>();
            }
        }
    }
}

pub fn check_interaction(
    mut interaction_query: Query<
        (&Interaction, &MainMenuButton),
        (Changed<Interaction>, With<MainMenuButton>),
    >,
    mut button_clicked_writer: EventWriter<MainMenuButtonClicked>,
) {
    for (interaction, button) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                button_clicked_writer.write(MainMenuButtonClicked(button.0));
            }
            _ => {}
        }
    }
}

pub fn check_keyboard_click(
    focus: Res<InputFocus>,
    button_input: Res<ButtonInput<KeyCode>>,
    query: Query<&MainMenuButton>,
    mut button_clicked_writer: EventWriter<MainMenuButtonClicked>,
) {
    if let Some(element) = focus.0 {
        if button_input.just_pressed(KeyCode::Enter) {
            if let Ok(button) = query.get(element) {
                button_clicked_writer.write(MainMenuButtonClicked(button.0));
            }
        }
    }
}

pub fn check_button_clicked(
    mut button_clicked_reader: EventReader<MainMenuButtonClicked>,
    mut exit_writer: EventWriter<AppExit>,
) {
    for event in button_clicked_reader.read() {
        match event.0 {
            MainMenuButtonAction::Exit => {
                exit_writer.write(AppExit::Success);
                ()
            }
            _ => (),
        }
    }
}
