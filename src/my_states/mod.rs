use bevy::prelude::*;

pub(crate) struct MyStatesPlugin;

impl Plugin for MyStatesPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<GameState>()
            .init_state::<GameState>()
            .enable_state_scoped_entities::<GameState>();
    }
}

#[derive(States, Clone, PartialEq, Eq, Hash, Debug, Default, Reflect)]
pub enum GameState {
    #[default]
    MainMenu,
    Started,
}
