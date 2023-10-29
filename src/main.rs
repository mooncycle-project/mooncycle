use crate::plugins::enemy::EnemyPlugin;
use crate::plugins::planet::PlanetPlugin;
use crate::plugins::player::PlayerPlugin;
use crate::plugins::score::ScorePlugin;
use bevy::prelude::*;
use bevy::render::camera::Viewport;
use bevy::window::PresentMode;
use bevy_rapier2d::prelude::*;
use std::ops::Add;

mod plugins;

const TIME_STEP: f32 = 1.0 / 60.0;

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
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .add_systems(Startup, setup)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        //.add_plugins(ArenaPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(EnemyPlugin)
        .add_plugins(ScorePlugin)
        .add_plugins(PlanetPlugin)
        .insert_resource(FixedTime::new_from_secs(TIME_STEP))
        .insert_resource(RapierConfiguration {
            gravity: Vec2::new(0.0, 0.0),
            ..default()
        })
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
}
