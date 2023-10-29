use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

const PLAYER_STARTING_POSITION: Vec3 = Vec3::new(0., 0., 0.);
const PLAYER_RADIUS: f32 = 50.;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, player_movement);
    }
}

#[derive(Component)]
pub struct Player;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((Player, RigidBody::Dynamic))
        .insert(Collider::ball(PLAYER_RADIUS))
        .insert(ColliderMassProperties::Density(2.0))
        .insert(ColliderMassProperties::Mass(10.0))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(SpriteBundle {
            texture: asset_server.load("textures/moon.png"),
            sprite: Sprite {
                custom_size: Some(Vec2::new(PLAYER_RADIUS * 2.2, PLAYER_RADIUS * 2.2)),
                ..default()
            },
            ..default()
        })
        .insert(TransformBundle::from(Transform::from_translation(
            PLAYER_STARTING_POSITION,
        )))
        .insert(Ccd::enabled())
        .insert(Sleeping::disabled())
        .insert(Velocity {
            linvel: Vec2::new(0., 0.),
            angvel: 0.0,
        });
}

fn player_movement(
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut Velocity, With<Player>>,
) {
    let mut velocity = query.single_mut();

    if (keys.pressed(KeyCode::W) || keys.pressed(KeyCode::Up)) && velocity.linvel.y < 1500.0 {
        velocity.linvel.y += 1500. * time.delta_seconds();
    }

    if (keys.pressed(KeyCode::S) || keys.pressed(KeyCode::Down)) && velocity.linvel.y > -1500.0 {
        velocity.linvel.y -= 1500. * time.delta_seconds();
    }

    if (keys.pressed(KeyCode::A) || keys.pressed(KeyCode::Left)) && velocity.linvel.x > -1500.0 {
        velocity.linvel.x -= 1500. * time.delta_seconds();
    }

    if (keys.pressed(KeyCode::D) || keys.pressed(KeyCode::Right)) && velocity.linvel.x < 1500.0 {
        velocity.linvel.x += 1500. * time.delta_seconds();
    }
}
