use bevy::prelude::*;

use crate::resources::Scoreboard;
use crate::components::Scoreboard as ScoreboardComponent;

pub fn update_scoreboard(scoreboard: Res<Scoreboard>, mut query: Query<&mut Text, With<ScoreboardComponent>>) {
    let mut text = query.single_mut();
        text.sections[1].value = format!("{}", scoreboard.score);
}