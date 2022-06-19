use bevy::prelude::*;

use crate::{constants::*, components::Scoreboard};


pub fn setup_scoreboard(mut commands: Commands, asset_server: Res<AssetServer>){

		let style = TextStyle {
			font: asset_server.load("fonts/FiraSans-Bold.ttf"),
			font_size: SCOREBOARD_FONT_SIZE,
			color: TEXT_COLOR,
		};
	    // Scoreboard
		commands.spawn_bundle(TextBundle {
			text: Text {
				sections: vec![
					TextSection {
						value: "Score: ".to_string(),
						style: style.clone(),
					},
					TextSection {
						value: "".to_string(),
						style: style.clone(),
					},
				],
				..default()
			},
			style: Style {
				position_type: PositionType::Absolute,
				position: Rect {
					top: SCOREBOARD_TEXT_PADDING,
					left: SCOREBOARD_TEXT_PADDING,
					..default()
				},
				..default()
			},
			..default()
		}).insert(Scoreboard);
}