pub mod wall_bundle;

use bevy::{prelude::{Component, Deref, DerefMut}, math::Vec2};

#[derive(Component)]
pub struct Paddle;

#[derive(Component)]
pub struct Ball;

#[derive(Component, Deref, DerefMut)]
pub struct Velocity(Vec2);

#[derive(Component)]
pub struct Collider;

#[derive(Default)]
pub struct CollisionEvent;

#[derive(Component)]
pub struct Brick;
