use crate::plugins::enemy::EnemyDeathEvent;
use bevy::prelude::*;

const TEXT_COLOR: Color = Color::rgb(0.5, 0.5, 1.0);
const SCORE_COLOR: Color = Color::rgb(1.0, 0.5, 0.5);
const SCOREBOARD_FONT_SIZE: f32 = 40.0;
const SCOREBOARD_TEXT_PADDING: Val = Val::Px(5.0);

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Score(0))
            .add_systems(Startup, setup)
            .add_systems(Update, (update_score_on_enemy_death, update_scoreboard));
    }
}

#[derive(Resource)]
struct Score(u64);

fn setup(mut commands: Commands) {
    commands.spawn(
        TextBundle::from_sections([
            TextSection::new(
                "Score: ",
                TextStyle {
                    font_size: SCOREBOARD_FONT_SIZE,
                    color: TEXT_COLOR,
                    ..default()
                },
            ),
            TextSection::from_style(TextStyle {
                font_size: SCOREBOARD_FONT_SIZE,
                color: SCORE_COLOR,
                ..default()
            }),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: SCOREBOARD_TEXT_PADDING,
            left: SCOREBOARD_TEXT_PADDING,
            ..default()
        }),
    );
}

fn update_score_on_enemy_death(mut score: ResMut<Score>, mut events: EventReader<EnemyDeathEvent>) {
    for _ in events.iter() {
        score.0 += 1;
    }
}

fn update_scoreboard(scoreboard: Res<Score>, mut query: Query<&mut Text>) {
    let mut text = query.single_mut();
    text.sections[1].value = scoreboard.0.to_string();
}
