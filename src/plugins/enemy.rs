use bevy::prelude::*;
use bevy::time::common_conditions::on_fixed_timer;
use bevy_rapier2d::prelude::*;
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
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
            .add_event::<EnemyDeathEvent>()
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


#[derive(Event)]
pub struct EnemyDeathEvent;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(DeathSound(asset_server.load("sounds/death.ogg")));
}

fn enemy_spawner(mut commands: Commands, asset_server: Res<AssetServer>) {
    let spawn_point = *ENEMY_SPAWN_POINTS.choose(&mut thread_rng()).unwrap();


    commands
        .spawn((Enemy, RigidBody::Dynamic))
        .insert(Collider::ball(30.0))
        .insert(SpriteBundle {
            texture: asset_server.load("textures/shuriken.png"),
            sprite: Sprite {
                custom_size: Some(Vec2::new(60.0, 60.0)),
                ..default()
            },
            transform: Transform::from_translation(spawn_point),
                ..default()
        })
        .insert(TransformBundle::from(Transform::from_translation(spawn_point)))
        .insert(Velocity {
            linvel: Vec2::new(
                thread_rng().gen_range(-600.0..=600.0),
                thread_rng().gen_range(-600.0..=600.0),
            ),
            angvel: 360.0,
        })
        .insert(GravityScale(0.5))
        .insert(Sleeping::disabled())
        .insert(Ccd::enabled());
}

fn mark_dead_enemies(
    mut commands: Commands,
    enemies: Query<(Entity, &Velocity), (With<Enemy>, Without<Dead>)>,
    mut death_events: EventWriter<EnemyDeathEvent>,
) {
    for (entity, velocity) in enemies.iter() {
        if velocity.angvel < 0.1 {
            death_events.send(EnemyDeathEvent);
            commands.entity(entity).insert(Dead);
        }
    }
}

fn play_death_sound(
    mut commands: Commands,
    mut death_events: EventReader<EnemyDeathEvent>,
    death_sound: Res<DeathSound>,
) {
    if !death_events.is_empty() {
        commands.spawn(AudioBundle {
            source: death_sound.0.clone(),
            ..default()
        });
    }
}

fn despawn_dead_enemies(mut commands: Commands, dead_enemies: Query<Entity, With<Dead>>) {
    for entity in dead_enemies.iter() {
        commands.entity(entity).despawn();
    }
}
