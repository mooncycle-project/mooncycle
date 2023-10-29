use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

const PLANET_RADIUS: f32 = 150.0;

pub struct PlanetPlugin;

impl Plugin for PlanetPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

#[derive(Component)]
struct Planet;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((Planet, RigidBody::Fixed))
        .insert(GravityScale(10.0))
        .insert(Collider::ball(PLANET_RADIUS))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(SpriteBundle {
            texture: asset_server.load("textures/planet.png"),
            sprite: Sprite {
                custom_size: Some(Vec2::new(PLANET_RADIUS * 2.2, PLANET_RADIUS * 2.2)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
            ..Default::default()
        });
}
