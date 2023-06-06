use bevy::prelude::*;

#[derive(Component)]
pub struct ScoreDisplay;

#[derive(Component)]
pub struct BestScoreDisplay;

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_ui);
    }
}

fn setup_ui(mut cmds: Commands) {
    cmds.spawn(NodeBundle {
        ..Default::default()
    })
    .with_children(|parent| {
        parent.spawn(TextBundle {
            ..Default::default()
        });
        parent
            .spawn(NodeBundle {
                ..Default::default()
            })
            .with_children(|parent| {
                // score box
                parent
                    .spawn(NodeBundle {
                        ..Default::default()
                    })
                    .with_children(|parent| {
                        parent.spawn(TextBundle {
                            ..Default::default()
                        });
                        parent
                            .spawn(TextBundle {
                                ..Default::default()
                            })
                            .insert(ScoreDisplay);
                    });

                // best score box
            });
        parent
            .spawn(ButtonBundle {
                ..Default::default()
            })
            .with_children(|parent| {
                parent.spawn(TextBundle {
                    ..Default::default()
                });
            });
    });
}
