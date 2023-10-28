use bevy::prelude::*;
use bevy::time::common_conditions::on_fixed_timer;
use bevy_rapier2d::prelude::*;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::time::Duration;

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
        app.add_systems(Startup, setup)
            .add_systems(
                FixedUpdate,
                enemy_spawner.run_if(on_fixed_timer(Duration::from_secs(1))),
            )
            .add_systems(
                Update,
                (mark_dead_enemies, play_death_sound, despawn_dead_enemies),
            );
    }
}

#[derive(Resource)]
struct DeathSound(Handle<AudioSource>);

#[derive(Component)]
struct Enemy;

#[derive(Component)]
struct Dead;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(DeathSound(
        asset_server.load("sounds/death.ogg"),
    ));
}

fn enemy_spawner(mut commands: Commands) {
    commands
        .spawn((Enemy, RigidBody::Dynamic))
        .insert(Collider::cuboid(30.0, 30.0))
        .insert(TransformBundle::from(Transform::from_translation(
            *ENEMY_SPAWN_POINTS.choose(&mut thread_rng()).unwrap(),
        )))
        .insert(Velocity {
            linvel: Vec2::new(100., 100.),
            angvel: 360.,
        })
        .insert(GravityScale(0.5))
        .insert(Sleeping::disabled())
        .insert(Ccd::enabled());
}

fn mark_dead_enemies(
    mut commands: Commands,
    enemies: Query<(Entity, &Velocity), (With<Enemy>, Without<Dead>)>,
) {
    for (entity, velocity) in enemies.iter() {
        if velocity.angvel < 0.5 {
            commands.entity(entity).insert(Dead);
        }
    }
}

fn play_death_sound(
    mut commands: Commands,
    death_sound: Res<DeathSound>,
    dead_enemies: Query<Entity, (With<Enemy>, With<Dead>)>,
) {
    for _ in dead_enemies.iter() {
        commands.spawn(AudioBundle {
            source: death_sound.0.clone(),
            settings: PlaybackSettings::DESPAWN,
        });
    }
}

fn despawn_dead_enemies(mut commands: Commands, dead_enemies: Query<Entity, With<Dead>>) {
    for entity in dead_enemies.iter() {
        commands.entity(entity).despawn();
    }
}