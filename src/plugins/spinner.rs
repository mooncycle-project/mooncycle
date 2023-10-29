use std::f32::consts::PI;
use bevy::prelude::*;
use bevy_rapier2d::prelude::Velocity;

pub struct SpinnerPlugin {
    pub debug: bool,
}

// TODO maybe max angvel can be different for each spinner
const MAX_ANGVEL:f32 = 10.0 * PI;

#[derive(Component)]
pub struct Spinner {
    /// tilt in percent (0..1)
    pub tilt: Vec2,
    pub radius: f32,
}

impl Default for SpinnerPlugin {
    fn default() -> Self {
        Self {
            debug: true
        }
    }
}

impl Plugin for SpinnerPlugin {
    fn build(&self, app: &mut App) {
        if self.debug {
            // TODO find better schedule between rapier calculated the velocity and the velocity is applied to the global transofmration
            app.add_systems(Update, apply_limits);
            app.add_systems(PostUpdate, render_spinner_debug);
        }
    }
}

fn apply_limits(
    mut spinners: Query<&mut Velocity, With<Spinner>>
) {
    for mut velocity in spinners.iter_mut() {
        velocity.angvel = velocity.angvel.clamp(-MAX_ANGVEL, MAX_ANGVEL);
    }
}

fn render_spinner_debug(
    mut gizmos: Gizmos,
    spinners: Query<(&Spinner, &Transform, &Velocity)>
) {
    for (spinner, transform, velocity) in spinners.iter() {
        let pos = transform.translation.truncate();
        let health = (velocity.angvel / MAX_ANGVEL).abs().min(1.0);

        // tilt
        gizmos.line_2d(
            pos,
            pos + spinner.tilt * 30.0,
            Color::LIME_GREEN);

        // health bar
        let health_bar_y = pos.y + spinner.radius + 10.0;
        gizmos.line_2d(
            Vec2::new(pos.x - spinner.radius, health_bar_y),
            Vec2::new(pos.x - spinner.radius + spinner.radius * 2.0 * health, health_bar_y),
            Color::LIME_GREEN);
        gizmos.rect_2d(
            Vec2::new(pos.x, health_bar_y),
            0.0,
            Vec2::new(spinner.radius * 2.0, 3.0),
            Color::GRAY);
    }
}