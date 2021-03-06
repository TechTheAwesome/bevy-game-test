pub mod wall_bundle;

use bevy::{prelude::{Component, Deref, DerefMut}, math::Vec2};

#[derive(Component)]
pub struct Paddle;

#[derive(Component)]
pub struct Ball;

#[derive(Component)]
pub struct Scoreboard;

#[derive(Component)]
pub struct Button;


#[derive(Component, Deref, DerefMut)]
pub struct Velocity(pub Vec2);

#[derive(Component)]
pub struct Collider;

#[derive(Component)]
pub struct Brick;

#[derive(Debug)]
pub enum CollisionEvent {
	Wall,
	Brick,
	Paddle
}

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct StartMenu;

#[derive(Component)]
pub struct StartButton;