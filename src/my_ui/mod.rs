use bevy::{color::palettes::css, prelude::*};
use lightyear::prelude::{client::ClientCommands, server::ServerCommands};

use crate::{lightyear::lib::MyNetConfigControl, my_states::GameState};

pub struct MyUiPlugin;

impl Plugin for MyUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::MainMenu), setup_ui)
            .add_systems(
                Update,
                ui_interaction_system.run_if(in_state(GameState::MainMenu)),
            );
    }
}

fn ui_interaction_system(
    mut commands: Commands,
    mut interaction_query: Query<
        (Entity, &Interaction, &mut BackgroundColor),
        Changed<Interaction>,
    >,
) {
    for (button_entity, interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = css::GREEN.into();

                commands.trigger_targets(ButtonPressedTrigger, button_entity);
            }
            Interaction::Hovered => {
                *color = css::BLUE.into();
            }
            Interaction::None => {
                *color = css::GRAY.into();
            }
        }
    }
}

#[derive(Event)]
struct ButtonPressedTrigger;

fn setup_ui(mut commands: Commands) {
    commands
        .spawn((
            StateScoped(GameState::MainMenu),
            Name::new("UiContainer"),
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    row_gap: Val::Px(10.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|commands| {
            commands
                .spawn((ButtonBundle {
                    style: Style {
                        width: Val::Px(150.0),
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(5.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: BorderColor(Color::BLACK),
                    background_color: css::GRAY.into(),
                    ..default()
                },))
                .observe(
                    |_: Trigger<ButtonPressedTrigger>,
                     mut commands: Commands,
                     mut network: MyNetConfigControl| {
                        network.set_to_host();
                        commands.start_server();
                        //commands.connect_client();
                    },
                )
                .with_children(|commands| {
                    commands.spawn((TextBundle::from_section(
                        "Host",
                        TextStyle {
                            font_size: 30.0,
                            color: css::WHITE.into(),
                            ..default()
                        },
                    ),));
                });

            commands
                .spawn((ButtonBundle {
                    style: Style {
                        width: Val::Px(150.0),
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(5.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: BorderColor(Color::BLACK),
                    background_color: css::GRAY.into(),
                    ..default()
                },))
                .observe(
                    |_: Trigger<ButtonPressedTrigger>,
                     mut commands: Commands,
                     mut network: MyNetConfigControl| {
                        network.set_to_join();
                        commands.connect_client();
                    },
                )
                .with_children(|commands| {
                    commands.spawn((TextBundle::from_section(
                        "Connect",
                        TextStyle {
                            font_size: 30.0,
                            color: css::WHITE.into(),
                            ..default()
                        },
                    ),));
                });
        });
}
