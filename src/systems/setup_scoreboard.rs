use bevy::prelude::*;

use crate::constants::*;


pub fn setup_scoreboard(mut commands: Commands, asset_server: Res<AssetServer>){
	    // Scoreboard
		commands.spawn_bundle(TextBundle {
			text: Text {
				sections: vec![
					TextSection {
						value: "Score: ".to_string(),
						style: TextStyle {
							font: asset_server.load("fonts/FiraSans-Bold.ttf"),
							font_size: SCOREBOARD_FONT_SIZE,
							color: TEXT_COLOR,
						},
					},
					TextSection {
						value: "".to_string(),
						style: TextStyle {
							font: asset_server.load("fonts/FiraMono-Medium.ttf"),
							font_size: SCOREBOARD_FONT_SIZE,
							color: SCORE_COLOR,
						},
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
		});
}