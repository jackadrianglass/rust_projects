mod ui;

use ui::*;
use bevy::prelude::*;
use itertools::Itertools;
use rand::seq::IteratorRandom;
use std::{cmp::Ordering, collections::HashMap};

const TILE_SIZE: f32 = 40.0;
const TILE_SPACER: f32 = 10.0;

#[derive(Resource, Default)]
struct Game {
    score: u32,
}

#[derive(States, Default, Clone, Hash, Debug, PartialEq, Eq)]
enum GameState {
    #[default]
    Playing,
    GameOver,
}

#[derive(Component, Debug)]
struct NewTileEvent {}

#[derive(Component, Debug, PartialEq)]
struct Points {
    value: u32,
}

#[derive(Component, Debug, PartialEq, Eq, Hash)]
struct Position {
    x: u8,
    y: u8,
}

#[derive(Component)]
struct TileText {}

#[derive(Resource)]
struct FontSpec {
    family: Handle<Font>,
}

impl FromWorld for FontSpec {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.get_resource::<AssetServer>().unwrap();
        Self {
            family: asset_server.load("fonts/HeavyData.ttf"),
        }
    }
}

#[derive(Component)]
struct Board {
    size: u8,
}

impl Board {
    fn num_tiles(&self) -> usize {
        return (self.size * self.size).into();
    }
}

impl Board {
    fn board_size(&self) -> f32 {
        f32::from(self.size) * TILE_SIZE + f32::from(self.size + 1) * TILE_SPACER
    }

    fn cell_position_to_physical(&self, pos: u8) -> f32 {
        let offset = (TILE_SIZE - self.board_size()) * 0.5;
        offset + f32::from(pos) * TILE_SIZE + f32::from(pos + 1) * TILE_SPACER
    }
}

enum BoardShift {
    Up,
    Down,
    Left,
    Right,
}

impl TryFrom<&KeyCode> for BoardShift {
    type Error = &'static str;

    fn try_from(value: &KeyCode) -> Result<Self, Self::Error> {
        match value {
            KeyCode::Up => Ok(BoardShift::Up),
            KeyCode::Down => Ok(BoardShift::Down),
            KeyCode::Left => Ok(BoardShift::Left),
            KeyCode::Right => Ok(BoardShift::Right),
            _ => Err("Key code is not convertable into a board shift"),
        }
    }
}

impl BoardShift {
    fn sort(&self, a: &Position, b: &Position) -> Ordering {
        match *self {
            BoardShift::Left => match Ord::cmp(&a.y, &b.y) {
                Ordering::Equal => Ord::cmp(&a.x, &b.x),
                ordering => ordering,
            },
            BoardShift::Right => match Ord::cmp(&b.y, &a.y) {
                Ordering::Equal => Ord::cmp(&b.x, &a.x),
                ordering => ordering,
            },
            BoardShift::Down => match Ord::cmp(&a.x, &b.x) {
                Ordering::Equal => Ord::cmp(&a.y, &b.y),
                ordering => ordering,
            },
            BoardShift::Up => match Ord::cmp(&b.x, &a.x) {
                Ordering::Equal => Ord::cmp(&b.y, &a.y),
                ordering => ordering,
            },
        }
    }

    fn set_column_pos(&self, board_size: u8, pos: &mut Mut<Position>, idx: u8) {
        match self {
            BoardShift::Left => pos.x = idx,
            BoardShift::Right => pos.x = board_size - 1 - idx,
            BoardShift::Down => pos.y = idx,
            BoardShift::Up => pos.y = board_size - 1 - idx,
        }
    }

    fn get_row_pos(&self, pos: &Position) -> u8 {
        match self {
            BoardShift::Left | BoardShift::Right => pos.y,
            BoardShift::Up | BoardShift::Down => pos.x,
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(GameUiPlugin)
        .init_resource::<FontSpec>()
        .init_resource::<Game>()
        .add_event::<NewTileEvent>()
        .add_state::<GameState>()
        .add_startup_system(setup)
        .add_startup_system(spawn_board)
        .add_startup_system(spawn_tiles.in_base_set(StartupSet::PostStartup))
        .add_system(render_tile_points.in_set(OnUpdate(GameState::Playing)))
        .add_system(board_shift.in_set(OnUpdate(GameState::Playing)))
        .add_system(render_tiles.in_set(OnUpdate(GameState::Playing)))
        .add_system(new_tile_handler.in_set(OnUpdate(GameState::Playing)))
        .add_system(game_over.in_set(OnUpdate(GameState::Playing)))
        .run()
}

fn setup(mut cmds: Commands) {
    cmds.spawn(Camera2dBundle::default());
}

fn spawn_board(mut cmds: Commands) {
    let board = Board { size: 4 };
    let board_size = board.board_size();

    cmds.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::LIME_GREEN,
            custom_size: Some(Vec2::new(board_size, board_size)),
            ..default()
        },
        ..default()
    })
    .with_children(|builder| {
        for (x, y) in (0..board.size).cartesian_product(0..board.size) {
            builder.spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::GREEN,
                    custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                    ..default()
                },
                transform: Transform::from_xyz(
                    board.cell_position_to_physical(x),
                    board.cell_position_to_physical(y),
                    1.0,
                ),
                ..default()
            });
        }
    })
    .insert(board);
}

fn spawn_tiles(mut cmds: Commands, font_spec: Res<FontSpec>, query_board: Query<&Board>) {
    let board = query_board.single();
    let mut rng = rand::thread_rng();
    let starting_tiles: Vec<(u8, u8)> = (0..board.size)
        .cartesian_product(0..board.size)
        .choose_multiple(&mut rng, 2);
    for (x, y) in starting_tiles.iter() {
        let pos = Position { x: *x, y: *y };
        spawn_tile(&mut cmds, &font_spec, &board, pos);
    }
}

fn render_tile_points(
    mut texts: Query<&mut Text, With<TileText>>,
    tiles: Query<(&Points, &Children)>,
) {
    for (points, children) in tiles.iter() {
        if let Some(entity) = children.first() {
            let mut text = texts
                .get_mut(*entity)
                .expect("The first child of a point should be text");
            let mut text_section = text
                .sections
                .first_mut()
                .expect("expect that the first section of text to be mutable");
            text_section.value = points.value.to_string();
        }
    }
}

fn board_shift(
    mut cmds: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    mut tiles: Query<(Entity, &mut Points, &mut Position)>,
    query_board: Query<&Board>,
    mut tile_writer: EventWriter<NewTileEvent>,
    mut game: ResMut<Game>,
) {
    let board = query_board.single();
    let maybe_board_shift = keyboard_input
        .get_just_pressed()
        .find_map(|key_code| BoardShift::try_from(key_code).ok());

    let Some(board_shift) = maybe_board_shift else {return;};
    let mut it = tiles
        .iter_mut()
        .sorted_by(|a, b| board_shift.sort(&a.2, &b.2))
        .peekable();

    let mut col = 0;
    while let Some(mut tile) = it.next() {
        board_shift.set_column_pos(board.size, &mut tile.2, col);
        let Some(tile_next) = it.peek() else { continue; };

        if board_shift.get_row_pos(&tile.2) != board_shift.get_row_pos(&tile_next.2) {
            // different rows
            col = 0;
        } else if tile.1.value != tile_next.1.value {
            col += 1;
        } else {
            let tile_next_instance = it.next().unwrap();
            tile.1.value *= 2;
            game.score += tile.1.value;

            cmds.entity(tile_next_instance.0).despawn_recursive();
            if let Some(future) = it.peek() {
                if board_shift.get_row_pos(&tile.2) != board_shift.get_row_pos(&future.2) {
                    col = 0;
                } else {
                    col += 1;
                }
            }
        }
    }
    tile_writer.send(NewTileEvent {});
}

fn render_tiles(
    mut tiles: Query<(&mut Transform, &Position, Changed<Position>)>,
    query_board: Query<&Board>,
) {
    let board = query_board.single();
    for (mut transform, pos, pos_changed) in tiles.iter_mut() {
        if pos_changed {
            transform.translation.x = board.cell_position_to_physical(pos.x);
            transform.translation.y = board.cell_position_to_physical(pos.y);
        }
    }
}

fn new_tile_handler(
    mut tile_reader: EventReader<NewTileEvent>,
    mut cmds: Commands,
    query_board: Query<&Board>,
    tiles: Query<&Position>,
    font_spec: Res<FontSpec>,
) {
    let board = query_board.single();

    for _ in tile_reader.iter() {
        let mut rng = rand::thread_rng();
        let possible_position: Option<Position> = (0..board.size)
            .cartesian_product(0..board.size)
            .filter_map(|tile_pos| {
                let pos = Position {
                    x: tile_pos.0,
                    y: tile_pos.1,
                };
                match tiles.iter().find(|p| **p == pos) {
                    Some(_) => None,
                    None => Some(pos),
                }
            })
            .choose(&mut rng);
        if let Some(pos) = possible_position {
            spawn_tile(&mut cmds, &font_spec, &board, pos);
        }
    }
}

fn spawn_tile(cmds: &mut Commands, font_spec: &Res<FontSpec>, board: &Board, pos: Position) {
    cmds.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::BLUE,
            custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
            ..default()
        },
        transform: Transform::from_xyz(
            board.cell_position_to_physical(pos.x),
            board.cell_position_to_physical(pos.y),
            2.0,
        ),
        ..default()
    })
    .with_children(|child_builder| {
        child_builder
            .spawn(Text2dBundle {
                text: Text::from_section(
                    "4",
                    TextStyle {
                        font: font_spec.family.clone(),
                        font_size: 40.0,
                        color: Color::WHITE,
                    },
                )
                .with_alignment(TextAlignment::Center),
                transform: Transform::from_xyz(0.0, 0.0, 2.0),
                ..Default::default()
            })
            .insert(TileText {});
    })
    .insert(Points { value: 2 })
    .insert(pos);
}

fn game_over(query_board: Query<&Board>, tiles: Query<(&Position, &Points)>, mut run_state: ResMut<NextState<GameState>>) {
    let board = query_board.single();
    let tile_map: HashMap<&Position, &Points> = tiles.iter().collect();

    if tile_map.len() != board.num_tiles() { return; }

    let neighbor_points = [(-1, 0), (1, 0), (0, 1), (0, -1)];
    let board_range = 0..(board.size as i8);

    let has_move = tiles.iter().any(|(pos, value)|{
        neighbor_points.iter().filter_map(|(x, y)| {
            let new_x = pos.x as i8 + x;
            let new_y = pos.y as i8 + y;

            if !(board_range.contains(&new_x) && board_range.contains(&new_y)) {
                return None;
            }

            tile_map.get(&Position{
                x: new_x.try_into().unwrap(),
                y: new_y.try_into().unwrap(),
            })
        }).any(|v| *v == value)
    });

    if !has_move {
        dbg!("GAME OVER!");
        run_state.set(GameState::GameOver);
    }
}
