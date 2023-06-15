use bevy::prelude::*;

use crate::{FontSpec, Game, GameState};

#[derive(Component)]
pub struct ScoreDisplay;

#[derive(Component)]
pub struct BestScoreDisplay;

#[derive(Component)]
pub struct GameRestartButton;

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_ui)
            .add_system(score_board)
            .add_system(button_interaction_system)
            .add_system(button_text);
    }
}

fn setup_ui(mut cmds: Commands, font_spec: Res<FontSpec>) {
    cmds.spawn(NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            align_items: AlignItems::FlexEnd,
            // padding: Rect::all(Val::Px(50.0)),
            ..Default::default()
        },
        ..Default::default()
    })
    .with_children(|parent| {
        parent.spawn(TextBundle {
            text: Text::from_section(
                "2048",
                TextStyle {
                    font: font_spec.family.clone(),
                    font_size: 40.0,
                    color: Color::WHITE,
                },
            ),
            ..Default::default()
        });

        parent
            .spawn(NodeBundle {
                style: Style {
                    justify_content: JustifyContent::Center,
                    size: Size::new(Val::Percent(100.0), Val::Auto),
                    ..Default::default()
                },
                ..Default::default()
            })
            .with_children(|parent| {
                // scorebox
                parent
                    .spawn(NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::ColumnReverse,
                            align_items: AlignItems::Center,
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .with_children(|parent| {
                        parent.spawn(TextBundle {
                            text: Text::from_section(
                                "Score",
                                TextStyle {
                                    font: font_spec.family.clone(),
                                    font_size: 15.0,
                                    color: Color::WHITE,
                                },
                            ),
                            ..Default::default()
                        });
                        parent
                            .spawn(TextBundle {
                                text: Text::from_section(
                                    "<score>",
                                    TextStyle {
                                        font: font_spec.family.clone(),
                                        font_size: 20.0,
                                        color: Color::WHITE,
                                    },
                                ),
                                ..Default::default()
                            })
                            .insert(ScoreDisplay);
                    });
                // end scorebox
                // best scorebox
                parent
                    .spawn(NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::ColumnReverse,
                            align_items: AlignItems::Center,
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .with_children(|parent| {
                        parent.spawn(TextBundle {
                            text: Text::from_section(
                                "Best",
                                TextStyle {
                                    font: font_spec.family.clone(),
                                    font_size: 15.0,
                                    color: Color::WHITE,
                                },
                            ),
                            ..Default::default()
                        });
                        parent
                            .spawn(TextBundle {
                                text: Text::from_section(
                                    "<score>",
                                    TextStyle {
                                        font: font_spec.family.clone(),
                                        font_size: 20.0,
                                        color: Color::WHITE,
                                    },
                                ),
                                ..Default::default()
                            })
                            .insert(BestScoreDisplay);
                    });
                // end best scorebox
            });
        parent
            .spawn(ButtonBundle {
                style: Style {
                    size: Size::new(Val::Px(100.0), Val::Px(30.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                ..Default::default()
            })
            .with_children(|parent| {
                parent
                    .spawn(TextBundle {
                        text: Text::from_section(
                            "Button",
                            TextStyle {
                                font: font_spec.family.clone(),
                                font_size: 20.0,
                                color: Color::WHITE,
                            },
                        ),
                        ..Default::default()
                    })
                    .insert(GameRestartButton);
            });
    });
}

fn score_board(
    game: Res<Game>,
    mut query_score: ParamSet<(
        Query<&mut Text, With<ScoreDisplay>>,
        Query<&mut Text, With<BestScoreDisplay>>,
    )>
) {
    let mut p0 = query_score.p0();
    if let Some(mut text) = p0.iter_mut().next() {
        text.sections[0].value = game.score.to_string();
    }

    let mut p1 = query_score.p1();
    if let Some(mut text) = p1.iter_mut().next() {
        text.sections[0].value = game.best_score.to_string();
    }
}

fn button_interaction_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    current_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (interaction, mut bg_colour) in interaction_query.iter_mut() {
        match interaction {
            Interaction::None => {
                *bg_colour = BackgroundColor(Color::rgb(0.8, 0.8, 0.8));
            }
            Interaction::Hovered => {
                *bg_colour = BackgroundColor(Color::rgb(0.9, 0.9, 0.9));
            }
            Interaction::Clicked => {
                *bg_colour = BackgroundColor(Color::rgb(0.7, 0.7, 0.7));
                match current_state.0 {
                    GameState::GameOver => next_state.set(GameState::Playing),
                    GameState::Playing => next_state.set(GameState::GameOver),
                }
            }
        }
    }
}

fn button_text(
    current_state: Res<State<GameState>>,
    mut game_restart_button: Query<&mut Text, With<GameRestartButton>>,
) {
    let mut button_content = game_restart_button.single_mut();
    match current_state.0 {
        GameState::Playing => button_content.sections[0].value = "Stop".to_string(),
        GameState::GameOver => button_content.sections[0].value = "Restart".to_string(),
    }
}
