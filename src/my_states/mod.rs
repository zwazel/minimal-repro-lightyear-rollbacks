use bevy::prelude::*;

pub(crate) struct MyStatesPlugin;

impl Plugin for MyStatesPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<GameState>()
            .register_type::<InGameUnpaused>()
            .register_type::<InGamePaused>()
            .init_state::<GameState>()
            .add_computed_state::<InGameUnpaused>()
            .add_computed_state::<InGamePaused>()
            .enable_state_scoped_entities::<GameState>()
            .enable_state_scoped_entities::<InGamePaused>()
            .enable_state_scoped_entities::<InGameUnpaused>();
    }
}

#[derive(States, Clone, PartialEq, Eq, Hash, Debug, Default, Reflect)]
pub enum GameState {
    #[default]
    MainMenu,
    Started {
        paused: bool,
    },
}

#[derive(Clone, PartialEq, Eq, Hash, Debug, Reflect, Default)]
pub(crate) struct InGameUnpaused;

impl ComputedStates for InGameUnpaused {
    type SourceStates = GameState;

    fn compute(sources: Self::SourceStates) -> Option<Self> {
        match sources {
            GameState::Started { paused: false, .. } => Some(Self),
            _ => None,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug, Reflect, Default)]
pub(crate) struct InGamePaused;

impl ComputedStates for InGamePaused {
    type SourceStates = GameState;

    fn compute(sources: Self::SourceStates) -> Option<Self> {
        match sources {
            GameState::Started { paused: true, .. } => Some(Self),
            _ => None,
        }
    }
}
