use crate::components::{Paddle, Collider};

use crate::constants::*;
use bevy::prelude::*;



pub fn setup_paddle(mut commands: Commands) {
	    // Paddle
		let paddle_y = BOTTOM_WALL + GAP_BETWEEN_PADDLE_AND_FLOOR;

		commands
			.spawn()
			.insert(Paddle)
			.insert_bundle(SpriteBundle {
				transform: Transform {
					translation: Vec3::new(0.0, paddle_y, 0.0),
					scale: PADDLE_SIZE,
					..default()
				},
				sprite: Sprite {
					color: PADDLE_COLOR,
					..default()
				},
				..default()
			})
			.insert(Collider);
}