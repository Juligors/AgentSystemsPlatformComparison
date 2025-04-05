use std::time::Duration;

use crate::config::Config;
use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;
use bevy::{core::TaskPoolThreadAssignmentPolicy, tasks::available_parallelism};

pub fn setup_basics(app: &mut App) {
    let default_plugins = setup_basic(app);

    #[cfg(feature = "headless")]
    setup_headless(app, default_plugins);

    #[cfg(feature = "windowed")]
    setup_windowed(app, default_plugins);
}

fn setup_basic(app: &mut App) -> PluginGroupBuilder {
    app.add_systems(PostUpdate, close_after_n_iterations);

    DefaultPlugins
        .set(TaskPoolPlugin {
            task_pool_options: TaskPoolOptions {
                compute: TaskPoolThreadAssignmentPolicy {
                    // set the minimum # of compute threads to the total number of available threads
                    min_threads: available_parallelism(),
                    // unlimited max threads
                    max_threads: usize::MAX,
                    // this value is irrelevant in this case
                    percent: 1.0,
                },
                ..default()
            },
        })
        .disable::<bevy::log::LogPlugin>()
    // .build()
}

#[cfg(feature = "headless")]
fn setup_headless(app: &mut App, default_plugins: PluginGroupBuilder) {
    app.add_plugins(default_plugins.build()).add_plugins(
        bevy::app::ScheduleRunnerPlugin::run_loop(core::time::Duration::from_secs_f32(
            f32::MIN_POSITIVE,
        )),
    );
}

#[cfg(feature = "windowed")]
fn setup_windowed(app: &mut App, default_plugins: PluginGroupBuilder) {
    use bevy::{
        window::{PresentMode, WindowTheme},
        winit::{UpdateMode, WinitSettings},
    };

    let config = app.world().get_resource::<Config>().expect(
        "Failed to get config resource, make sure its plugin is added before other plugins",
    );

    app.add_plugins(
        default_plugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Game of Life".into(),
                    resolution: (config.visuals.window_width, config.visuals.window_height).into(),
                    present_mode: PresentMode::AutoNoVsync,
                    window_theme: Some(WindowTheme::Dark),
                    position: WindowPosition::At((50, 50).into()),
                    ..default()
                }),
                ..default()
            })
            .build(),
    )
    .insert_resource(ClearColor(Color::srgb(0.7, 0.7, 0.7)))
    .insert_resource(WinitSettings {
        focused_mode: UpdateMode::Continuous,
        unfocused_mode: UpdateMode::Continuous,
    })
    .add_systems(Startup, setup_camera)
    .add_systems(Update, (camera_movement, camera_zoom, close_on_esc));
}

#[cfg(feature = "windowed")]
fn setup_camera(mut commands: Commands, config: Res<Config>) {
    let mut projection = OrthographicProjection::default_2d();
    projection.scale =
        config.height as f32 * config.visuals.cell_height / config.visuals.window_height * 1.1;

    commands.spawn((
        Camera2d,
        Transform::from_xyz(
            config.width as f32 * config.visuals.cell_width / 2.0,
            config.height as f32 * config.visuals.cell_height / 2.0,
            1000.0,
        ),
        projection,
    ));
}

#[cfg(feature = "windowed")]
fn camera_movement(
    camera: Single<(&mut Transform, &OrthographicProjection)>,
    mouse_motion: Res<bevy::input::mouse::AccumulatedMouseMotion>,
    mouse_button: Res<ButtonInput<MouseButton>>,
) {
    if !mouse_button.pressed(MouseButton::Right) {
        return;
    }

    let (mut transform, ortographic_projection) = camera.into_inner();
    let mut move_by = mouse_motion.delta.extend(0.0);
    move_by.x *= -1.0;
    move_by *= ortographic_projection.scale;
    transform.translation += move_by;
}

#[cfg(feature = "windowed")]
fn camera_zoom(
    mut ortographic_projection: Single<&mut OrthographicProjection>,
    mouse_wheel_input: Res<bevy::input::mouse::AccumulatedMouseScroll>,
) {
    let change = 0.1 * mouse_wheel_input.delta.y.clamp(-1.0, 1.0);
    let log_scale = ortographic_projection.scale.ln() - change;
    ortographic_projection.scale = log_scale.exp();
}

#[cfg(feature = "windowed")]
pub fn close_on_esc(
    mut commands: Commands,
    focused_windows: Query<(Entity, &Window)>,
    input: Res<ButtonInput<KeyCode>>,
) {
    for (entity, window) in focused_windows.iter() {
        if !window.focused {
            continue;
        }

        if input.just_pressed(KeyCode::Escape) {
            commands.entity(entity).despawn();
        }
    }
}

fn close_after_n_iterations(
    config: Res<Config>,
    mut timer: Local<Option<Timer>>,
    mut exit: EventWriter<AppExit>,
) {
    match timer.as_mut() {
        None => *timer = Timer::from_seconds(config.iterations as f32, TimerMode::Once).into(),
        Some(timer) => {
            if timer.tick(Duration::from_secs(1)).just_finished() {
                exit.send(AppExit::Success);
            }
        }
    };
}
