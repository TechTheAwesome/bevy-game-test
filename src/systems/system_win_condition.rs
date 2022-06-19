use bevy::prelude::{EventReader, Query, With};

use crate::components::{Ball, Brick, CollisionEvent, Velocity};

pub fn check_win_condition(
    mut collision_events: EventReader<CollisionEvent>,
    bricks: Query<&Brick>,
    mut ball: Query<&mut Velocity, With<Ball>>,
) {
    if collision_events.is_empty() {
        return;
    }
    eprintln!("{} collisions", collision_events.len());
    for event in collision_events.iter() {
        match event {
            CollisionEvent::Wall => {}
            CollisionEvent::Brick => {
                if bricks.is_empty() {
                    println!("You WIN!");
                    let mut test = ball.single_mut();
                    test.x = 0.;
                    test.y = 0.;
                    return;
                }
                println!("{} bricks left", bricks.iter().len());
            }
            CollisionEvent::Paddle => {}
        }
    }
}
