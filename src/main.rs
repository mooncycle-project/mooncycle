use crate::plugins::arena::ArenaPlugin;
use crate::plugins::enemy::EnemyPlugin;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use std::ops::Add;

mod plugins;

const TIME_STEP: f32 = 1.0 / 60.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(ArenaPlugin)
        .add_plugins(EnemyPlugin)
        .add_plugins(RapierDebugRenderPlugin::default())
        .insert_resource(FixedTime::new_from_secs(TIME_STEP))
        .insert_resource(RapierConfiguration {
            gravity: Vec2::new(0.0, 0.0),
            ..default()
        })
        .add_systems(Startup, setup)
        .add_systems(Startup, setup_physics)
        .add_systems(FixedUpdate, (player_movement_system, apply_forces))
        .run();
}

#[derive(Component)]
struct Spinner {
    /// tilt in percent (0..1)
    tilt: Vec2,
    /// rotation speed in radians per second
    rotation_speed: f32,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());
}

/// Demonstrates applying rotation and movement based on keyboard input.
fn player_movement_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Spinner, &mut Transform, &mut Velocity)>,
) {
    let (mut spinner, mut transform, mut velocity) = query.single_mut();

    let mut tilt_x = 0.0;
    let mut tilt_y = 0.0;

    if keyboard_input.pressed(KeyCode::Up) {
        tilt_y += 0.1;
    }
    if keyboard_input.pressed(KeyCode::Down) {
        tilt_y -= 0.1;
    }
    if keyboard_input.pressed(KeyCode::Right) {
        tilt_x += 0.1;
    }
    if keyboard_input.pressed(KeyCode::Left) {
        tilt_x -= 0.1;
    }

    spinner.tilt = spinner
        .tilt
        .add(Vec2::new(tilt_x, tilt_y))
        .min(Vec2::ONE)
        .max(Vec2::NEG_ONE);
    velocity.linvel = velocity.linvel.add(spinner.tilt);
}

fn setup_physics(mut commands: Commands) {
    /* Create the bouncing ball. */
    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::ball(50.0))
        .insert(Spinner {
            rotation_speed: f32::to_radians(360.0),
            tilt: Vec2::new(0.0, 0.0),
        })
        .insert(Restitution::coefficient(0.7))
        .insert(Velocity {
            linvel: Vec2::ONE,
            angvel: 0.0,
        })
        .insert(TransformBundle::from(Transform::from_xyz(
            100.0, 300.0, 0.0,
        )))
        .insert(AdditionalMassProperties::Mass(1.0))
        .insert(ExternalForce {
            force: Vec2::new(0.0, 0.0),
            torque: 0.0,
        })
        .insert(Damping {
            linear_damping: 0.5,
            angular_damping: 10.0,
        });
}

fn apply_forces(mut ball: Query<(&Transform, &mut ExternalForce)>) {
    let (transform, mut force) = ball.single_mut();
    force.force = -transform.translation.truncate() * 0.5;
    force.torque = 0.4;
}
