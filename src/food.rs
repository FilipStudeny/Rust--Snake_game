use bevy::prelude::{Component, Color, Commands, Sprite, SpriteBundle, default};
use rand::prelude::random;
use crate::arena::*;


const FOOD_COLOR: Color = Color::rgb(1.0, 0.0, 1.0); 

#[derive(Component)]
pub struct Food;

pub fn spawn_snake_food(mut commands: Commands){
    commands.spawn_bundle(SpriteBundle{
        sprite: Sprite{
            color: FOOD_COLOR,
            ..default()
        },
        ..default()
    })
    .insert(Food)
    .insert(Position{
        _x: (random::<f32>() * ARENA_WIDTH as f32) as i32,
        _y: (random::<f32>() * ARENA_HEIGHT as f32) as i32,
    })
    .insert(Size::square(0.8));

}