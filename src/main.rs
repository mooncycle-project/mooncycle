use crate::plugins::arena::ArenaPlugin;
use crate::plugins::enemy::EnemyPlugin;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use std::ops::Add;
use crate::plugins::spinner::{Spinner, SpinnerPlugin};

mod plugins;

const TIME_STEP: f32 = 1.0 / 60.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(ArenaPlugin)
        .add_plugins(EnemyPlugin)
        .add_plugins(SpinnerPlugin { debug: true })
        .add_plugins(RapierDebugRenderPlugin::default())
        .insert_resource(FixedTime::new_from_secs(TIME_STEP))
        .insert_resource(RapierConfiguration {
            gravity: Vec2::new(0.0, 0.0),
            ..default()
        })
        .add_systems(Startup, setup)
        .add_systems(Startup, setup_physics)
        .add_systems(FixedUpdate, player_movement_system)
        .add_systems(Update, apply_forces)
        .run();
}


fn setup(
    mut commands: Commands,
) {
    commands.spawn(Camera2dBundle::default());
}

/// Demonstrates applying rotation and movement based on keyboard input.
fn player_movement_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Spinner, &mut Velocity)>,
) {
    let (mut spinner, mut velocity) = query.single_mut();

    let mut tilt_x = 0.0;
    let mut tilt_y = 0.0;
    let tilt_speed = 0.05;

    if keyboard_input.pressed(KeyCode::Up) {
        tilt_y += tilt_speed;
    }
    if keyboard_input.pressed(KeyCode::Down) {
        tilt_y -= tilt_speed;
    }
    if keyboard_input.pressed(KeyCode::Right) {
        tilt_x += tilt_speed;
    }
    if keyboard_input.pressed(KeyCode::Left) {
        tilt_x -= tilt_speed;
    }

    spinner.tilt = spinner
        .tilt
        .add(Vec2::new(tilt_x, tilt_y))
        .clamp_length_max(1.0);
    velocity.linvel = velocity.linvel.add(spinner.tilt * 10.0);
}

fn setup_physics(mut commands: Commands) {
    let radius = 50.0;
    /* Create the bouncing ball. */
    commands.spawn((
        Spinner {
            tilt: Vec2::new(0.0, 0.0),
            radius,
        },
        RigidBody::Dynamic,
        Collider::ball(radius),
        Restitution::coefficient(0.7),
        Velocity {
            linvel: Vec2::ZERO,
            angvel: 50.0,
        },
        TransformBundle::IDENTITY,
        AdditionalMassProperties::Mass(10.0),
        // ExternalForce {
        //     force: Vec2::new(0.0, 0.0),
        //     torque: 0.0,
        // },
        Damping {
            linear_damping: 0.0,
            angular_damping: 0.1,
        },
    ));
}

fn apply_forces(mut ball: Query<(&Transform, &mut ExternalForce)>) {
    for (transform, mut force) in ball.iter_mut() {
        force.force = -transform.translation.truncate() * 0.5;
    }
}