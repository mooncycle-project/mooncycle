use std::time::Duration;
use bevy::prelude::*;
use bevy::time::common_conditions::{on_fixed_timer};
use bevy_rapier2d::prelude::*;
use rand::thread_rng;
use rand::seq::SliceRandom;

const ENEMY_SPAWN_POINTS: &[Vec3] = &[
    Vec3::new(50., 50., 50.),
    Vec3::new(50., 50., -50.),
    Vec3::new(50., -50., 50.),
    Vec3::new(50., -50., -50.),
    Vec3::new(-50., 50., 50.),
    Vec3::new(-50., 50., -50.),
    Vec3::new(-50., -50., 50.),
    Vec3::new(-50., -50., -50.),
];

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(FixedUpdate, enemy_spawner.run_if(on_fixed_timer(Duration::from_secs(1))))
            .add_systems(Update, (mark_dead_enemies, despawn_dead_enemies));
    }
}

#[derive(Component)]
struct Enemy;

#[derive(Component)]
struct Dead;

fn enemy_spawner(mut commands: Commands) {
    commands
        .spawn((Enemy, RigidBody::Dynamic))
        .insert(Collider::cuboid(30.0, 30.0))
        .insert(TransformBundle::from(Transform::from_translation(*ENEMY_SPAWN_POINTS.choose(&mut thread_rng()).unwrap())))
        .insert(Velocity {
            linvel: Vec2::new(100., 100.),
            angvel: 360.,
        })
        .insert(GravityScale(0.5))
        .insert(Sleeping::disabled())
        .insert(Ccd::enabled());
}

fn mark_dead_enemies(mut commands: Commands, enemies: Query<(Entity, &Velocity), (With<Enemy>, Without<Dead>)>) {
    for (entity, velocity) in enemies.iter() {
        if velocity.angvel < 1. {
            commands.entity(entity).insert(Dead);
        }
    }
}

fn despawn_dead_enemies(mut commands: Commands, dead_enemies: Query<Entity, With<Dead>>) {
    for entity in dead_enemies.iter() {
        commands.entity(entity).despawn();
    }
}

fn enemy_movement() {}
