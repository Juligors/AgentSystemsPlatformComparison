use bevy::prelude::*;
use rayon::prelude::*;

use crate::config::Config;

pub struct GameOfLifePlugin;

impl Plugin for GameOfLifePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<StepEvent>()
            .add_systems(Startup, setup_board)
            .add_systems(Update, send_step_event);

        #[cfg(feature = "headless")]
        app.add_systems(Update, update_board);

        #[cfg(feature = "windowed")]
        app.add_systems(
            Update,
            (update_board, update_board_visuals)
                // .run_if(on_event::<StepEvent>)
                .chain(),
        );
    }
}

#[derive(Event)]
struct StepEvent;

fn send_step_event(keys: Res<ButtonInput<KeyCode>>, mut ew_step: EventWriter<StepEvent>) {
    if keys.just_pressed(KeyCode::Space) {
        ew_step.send(StepEvent);
    }
}

#[derive(Component)]
struct FirstBoardMarker;

#[derive(Resource, Deref, DerefMut)]
struct IsFirstBoardActive(bool);

#[derive(Component, Clone)]
struct Board {
    cells: Vec<Vec<Cell>>,
}

#[derive(Clone)]
struct Cell {
    alive: bool,
}

#[cfg(feature = "windowed")]
#[derive(Resource)]
struct CellAssets {
    alive_cell: Handle<ColorMaterial>,
    dead_cell: Handle<ColorMaterial>,
}

#[cfg(feature = "windowed")]
impl CellAssets {
    pub fn for_cell(&self, alive: bool) -> Handle<ColorMaterial> {
        if alive {
            self.alive_cell.clone()
        } else {
            self.dead_cell.clone()
        }
    }
}

#[derive(Resource)]
struct BoardVisuals {
    entities: Vec<Vec<Entity>>,
}

fn setup_board(
    mut commands: Commands,
    config: Res<Config>,
    #[cfg(feature = "windowed")] mut meshes: ResMut<Assets<Mesh>>,
    #[cfg(feature = "windowed")] mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut board = Board {
        cells: Vec::with_capacity(config.height as usize),
    };
    let mut board_visuals = BoardVisuals {
        entities: Vec::with_capacity(config.height as usize),
    };

    #[cfg(feature = "windowed")]
    let mesh = meshes.add(Rectangle::new(
        config.visuals.cell_width,
        config.visuals.cell_height,
    ));
    #[cfg(feature = "windowed")]
    let cell_assets = CellAssets {
        alive_cell: materials.add(Color::WHITE),
        dead_cell: materials.add(Color::BLACK),
    };

    for row in 0..config.height {
        let mut board_row = Vec::with_capacity(config.width as usize);
        let mut board_visuals_row = Vec::with_capacity(config.width as usize);
        for col in 0..config.width {
            let alive = config.initial_map[row as usize][col as usize];

            let entity = commands
                .spawn((
                    #[cfg(feature = "windowed")]
                    Mesh2d(mesh.clone()),
                    #[cfg(feature = "windowed")]
                    MeshMaterial2d(cell_assets.for_cell(alive)),
                    Transform::from_xyz(
                        col as f32 * config.visuals.cell_width,
                        row as f32 * config.visuals.cell_height,
                        0.0,
                    ),
                ))
                .id();

            board_row.push(Cell { alive });
            board_visuals_row.push(entity);
        }

        board.cells.push(board_row);
        board_visuals.entities.push(board_visuals_row);
    }

    commands.insert_resource(IsFirstBoardActive(true));
    commands.spawn((board.clone(), FirstBoardMarker));
    commands.spawn(board);

    #[cfg(feature = "windowed")]
    {
        commands.insert_resource(board_visuals);
        commands.insert_resource(cell_assets);
    }
}

fn update_board(
    mut first_board_query: Query<&mut Board, With<FirstBoardMarker>>,
    mut second_board_query: Query<&mut Board, Without<FirstBoardMarker>>,
    config: Res<Config>,
    mut is_first_board_active: ResMut<IsFirstBoardActive>,
) {
    let get_neighbours_positions = |x: i32, y: i32| {
        [
            (x, y + 1),
            (x + 1, y + 1),
            (x + 1, y),
            (x + 1, y - 1),
            (x, y - 1),
            (x - 1, y - 1),
            (x - 1, y),
            (x - 1, y + 1),
        ]
    };
    
    let normalize_position = |pos: &(i32, i32)| -> (usize, usize) {
        let (x, y) = *pos;

        let x = if x == -1 {
            config.width - 1
        } else if x == config.width {
            0
        } else {
            x
        };

        let y = if y == -1 {
            config.height - 1
        } else if y == config.height {
            0
        } else {
            y
        };

        (x as usize, y as usize)
    };

    let (prev_board, mut next_board) = if **is_first_board_active {
        (first_board_query.single(), second_board_query.single_mut())
    } else {
        (second_board_query.single(), first_board_query.single_mut())
    };
    **is_first_board_active = !**is_first_board_active;

    next_board
        .cells
        .par_iter_mut()
        .enumerate()
        .for_each(|(y, row)| {
            row.iter_mut().enumerate().for_each(|(x, cell)| {
                let alive_neighbours_count = get_neighbours_positions(x as i32, y as i32)
                    .iter()
                    .map(normalize_position)
                    .filter(|(x, y)| prev_board.cells[*y][*x].alive)
                    .count();

                cell.alive = if prev_board.cells[y][x].alive {
                    alive_neighbours_count == 2 || alive_neighbours_count == 3
                } else {
                    alive_neighbours_count == 3
                };
            });
        });
}

#[cfg(feature = "windowed")]
fn update_board_visuals(
    mut materials_query: Query<&mut MeshMaterial2d<ColorMaterial>>,
    cell_assets: Res<CellAssets>,
    board_visuals: Res<BoardVisuals>,
    first_board: Query<&Board, With<FirstBoardMarker>>,
    second_board: Query<&Board, Without<FirstBoardMarker>>,
    is_first_board_active: Res<IsFirstBoardActive>,
) {
    let board = if **is_first_board_active {
        first_board.single()
    } else {
        second_board.single()
    };

    for (i, row) in board_visuals.entities.iter().enumerate() {
        for (j, entity) in row.iter().enumerate() {
            materials_query
                .get_mut(*entity)
                .expect("Failed to fetch material for cell")
                .0 = cell_assets.for_cell(board.cells[i][j].alive);
        }
    }
}
