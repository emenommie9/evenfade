use bevy::state::state::States;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GlobalState {
    #[default]
    InitialLoading,
    MainMenu,
    AdventureLoading,
    AdventurePlaying,
}
