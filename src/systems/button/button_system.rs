use bevy::prelude::*;

use crate::{components::{StartMenu, StartButton}, GameState};

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

pub fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut UiColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

pub fn start_button(
    mut interaction_query: Query<
        &Interaction,
        (Changed<Interaction>, With<Button>, With<StartButton>),
    >,
    mut game_state: ResMut<State<GameState>>,
) {
    for interaction in interaction_query.iter_mut() {
        if let Interaction::Clicked = interaction {
            game_state
                .push(GameState::Playing)
                .expect("Can't push state on stack");
        }
    }
}

pub fn remove_button(mut commands: Commands, entities: Query<Entity, With<StartMenu>>) {
    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
