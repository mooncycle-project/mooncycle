use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

const WALL_RADIUS: f32 = 400.;
const WALL_NUM_SEGMENTS: u8 = 16;

pub struct ArenaPlugin {}

impl Plugin for ArenaPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands) {
    for i in 0..WALL_NUM_SEGMENTS {
        let radian: f32 = PI * 2.0 / (WALL_NUM_SEGMENTS as f32) * (i as f32);
        let size: f32 = PI * 2.0 * WALL_RADIUS / (WALL_NUM_SEGMENTS as f32);

        let mut pos = Transform::from_xyz(WALL_RADIUS * f32::cos(radian), WALL_RADIUS * f32::sin(radian), 0.0);
        pos.rotate_z(radian);

        /* Create the ground. */
        commands
            .spawn(Collider::cuboid(10.0, size / 2.0))
            .insert(TransformBundle::from(pos));
    }
}
