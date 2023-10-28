use crate::plugins::arena::ArenaPlugin;
use crate::plugins::enemy::EnemyPlugin;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use std::ops::Add;
use bevy::render::camera::Viewport;
use bevy::window::PresentMode;
use crate::plugins::player::PlayerPlugin;

mod plugins;

const TIME_STEP: f32 = 1.0 / 60.0;
const VIEWPORT_WIDTH: u32 = 2000;
const VIEWPORT_HEIGHT: u32 = 2000;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: PresentMode::AutoNoVsync, // Reduces input lag.
                fit_canvas_to_parent: true,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(ArenaPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(EnemyPlugin)
        .add_plugins(SpinnerPlugin { debug: true })
        .add_plugins(RapierDebugRenderPlugin::default())
        .insert_resource(FixedTime::new_from_secs(TIME_STEP))
        .insert_resource(RapierConfiguration {
            gravity: Vec2::new(0.0, 0.0),
            ..default()
        })
        .add_systems(Startup, setup)
        .run();
}


fn setup(
    mut commands: Commands,
) {
    commands.spawn(Camera2dBundle::default());
}
