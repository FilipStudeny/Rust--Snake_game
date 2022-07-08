use bevy::prelude::*;
use bevy::core::FixedTimestep;

mod snake;
mod arena;
mod food;

use crate::snake::*;
use crate::arena::{Size, Position, ARENA_COLOR, ARENA_HEIGHT, ARENA_WIDTH};
use crate::food::{ spawn_snake_food };


fn main() {

    const _SCREEN_WIDTH: f32 = 500.0;
    const _SCREEN_HEIGHT: f32 = 500.0;

    //CREATING NEW APP
    App::new()
    .insert_resource(ClearColor(ARENA_COLOR))
    .insert_resource(WindowDescriptor{
        title: "Snakey game !".to_string(),
        width: _SCREEN_WIDTH, height: _SCREEN_HEIGHT,
        ..default()
    })
    .add_startup_system(create_2d_camera)
    .add_startup_system(spawn_snake)
        .insert_resource(SnakeBody::default())
        .insert_resource(LastBodySegmentPosition::default())
    .add_event::<GrowthEvent>()
        .add_system(snake_input.before(snake_movement))
    .add_event::<SnakeDies>()
        .add_system_set(    //MOVE SNAKE EVERY SECOND
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.150))
                .with_system(snake_movement)
                .with_system(eat_food.after(snake_movement))
                .with_system(snake_growth.after(eat_food))
            
        )
        .add_system(snake_die.after(snake_movement))
        .add_system_set(    //SPAWN FOOD EVERY SECOND NOT FRAME
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(1.0))
                .with_system(spawn_snake_food)
            
        )
        .add_system_set_to_stage( //UPDATE AFTER UPDATE -> LATE UPDATE - LOAD STUFF AFTER UPDATE
            CoreStage::PostUpdate,
            SystemSet::new()
                .with_system(position_transformation)
                .with_system(size_scaling)
            )
    .add_plugins(DefaultPlugins)
    .add_system(bevy::input::system::exit_on_esc_system)
    .run();
}


fn create_2d_camera(mut commands: Commands){
    //COMMANDS - USED TO QUEUEUE COMMANDS
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}


//LOGIC -> IF WIDTH AND HEIGHT OF 1 IN GRID 10 AND WINDOW OF 1000PX -> FINAL SIZE OF 10PX
fn size_scaling(windows: Res<Windows>, mut query: Query<(&Size, &mut Transform)>){

    let window = windows.get_primary().unwrap();

    for (sprite_size, mut transform) in query.iter_mut() {
        transform.scale = Vec3::new(
            sprite_size._arena_width  / ARENA_WIDTH as f32 * window.width() as f32,
            sprite_size._arena_height / ARENA_HEIGHT as f32 * window.height() as f32,
            1.0,
        );
    }
}

/**
 * The position translation: if an item’s x coordinate is at 5 in our system, the width in our system is 10, 
 * and the window width is 200, then the coordinate should be 5 / 10 * 200 - 200 / 2. We subtract half the window 
 * width because our coordinate system starts at the bottom left, and Translation starts from the center. 
 * We then add half the size of a single tile, 
 * because we want our sprites bottom left corner to be at the bottom left of a tile, not the center.
 */

fn position_transformation(windows: Res<Windows>, mut query: Query<(&Position, &mut Transform)>){

    fn convert_coordinate(position: f32, bound_window: f32, bound_game: f32) -> f32 {
        let game_tile = bound_window / bound_game;
        position / bound_game * bound_window - (bound_window / 2.) + (game_tile / 2.)
    }

    let window = windows.get_primary().unwrap();
    for(pos, mut transform) in query.iter_mut(){
        transform.translation = Vec3::new(
            convert_coordinate(pos._x as f32, window.width() as f32, ARENA_WIDTH as f32),
            convert_coordinate(pos._y as f32, window.height() as f32, ARENA_HEIGHT as f32),
            0.0
        );
    }
}