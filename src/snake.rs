use bevy::input::Input;
use bevy::prelude::{
    Component, Color, Commands, Sprite, SpriteBundle, default, Deref, 
    DerefMut, Entity, ResMut, KeyCode, Query, Res, EventWriter, EventReader, With};
use crate::arena::*;
use crate::food::*;

const SNAKE_HEAD_COLOR: Color = Color::rgb(0.7, 0.7, 0.7);
const SNAKE_BODY_COLOR: Color = Color::rgb(0.3, 0.3, 0.3);

#[derive(Clone, Copy, PartialEq)]
pub enum MovementDirection{
    Left,
    Right,
    Up,
    Down
}

impl MovementDirection{
    fn reverse_direction(self) -> Self{
        match self{
            Self::Left => Self::Right,
            Self::Right => Self::Left,
            Self::Up => Self::Down,
            Self::Down => Self::Up
        }
    }
}

//EVENT COMPONENETS - SERVE AS SYSTEM FOR SENDING DATA BETWEAN DIFFERENT GAME SYSTEMS - FORX JUMP OR PICK UP
pub struct GrowthEvent;
pub struct SnakeDies;

#[derive(Component)]
pub struct SnakeSegment;

#[derive(Default, Deref, DerefMut)]
pub struct SnakeBody(Vec<Entity>);

#[derive(Default)]
pub struct LastBodySegmentPosition(Option<Position>);

#[derive(Component)]
pub struct SnakeHead{
    pub _movement_direction: MovementDirection,
}

pub fn spawn_snake(mut commands: Commands,  mut segments: ResMut<SnakeBody>){
    *segments = SnakeBody(vec![
        commands.spawn_bundle(SpriteBundle{
            sprite: Sprite{
                color: SNAKE_HEAD_COLOR,
                ..default()
            },
            ..default()
        })
        .insert(SnakeHead{
            _movement_direction: MovementDirection::Up,
        })
        .insert(SnakeSegment)
        .insert(Position { _x: 3, _y: 3})
        .insert(Size::square(0.8))
        .id(),
        spawn_snake_body_segment(commands, Position { _x: 3, _y: 3} )
    ]);
    
}

pub fn spawn_snake_body_segment(mut commands: Commands, position: Position) -> Entity{
    commands.spawn_bundle(SpriteBundle{
        sprite: Sprite { 
            color: SNAKE_BODY_COLOR,
            ..default() 
        },
        ..default()
    })
    .insert(SnakeSegment)
    .insert(position)
    .insert(Size::square(0.65))
    .id()
}

//SNAKE MOVEMENT - BY ITERETING OVER ENTITIES AND THEIR COMPONENETS 
pub fn snake_input(keyboard_input: Res<Input<KeyCode>>, mut heads: Query<&mut SnakeHead>){
   
    if let Some(mut head) = heads.iter_mut().next(){
        let look_direction: MovementDirection = 
            if keyboard_input.pressed(KeyCode::Left){
                MovementDirection::Left
            }else if keyboard_input.pressed(KeyCode::Right){
                MovementDirection::Right
            }else if keyboard_input.pressed(KeyCode::Up){
                MovementDirection::Up
            }else if keyboard_input.pressed(KeyCode::Down){
                MovementDirection::Down
            }else{
                head._movement_direction
            };

        if look_direction != head._movement_direction.reverse_direction(){
            head._movement_direction = look_direction
        }
    }
}

pub fn snake_movement(segments: ResMut<SnakeBody>, mut heads: Query<(Entity, &SnakeHead)>, 
                    mut positions: Query<&mut Position>, mut last_body_segment_position: ResMut<LastBodySegmentPosition>,
                    mut snake_dies_writer: EventWriter<SnakeDies>) {

    if let Some(( head_entity, head)) = heads.iter_mut().next(){
        let body_segment_positions = segments.iter().map(|e| * positions.get_mut(*e).unwrap())
        .collect::<Vec<Position>>();
        *last_body_segment_position = LastBodySegmentPosition(Some(*body_segment_positions.last().unwrap()));

        let mut head_position = positions.get_mut(head_entity).unwrap(); //get position from HEAD entity
        match &head._movement_direction{
            MovementDirection::Left => {
                head_position._x -= 1;
            }
            MovementDirection::Right => {
                head_position._x += 1;
            }
            MovementDirection::Up => {
                head_position._y += 1;
            }
            MovementDirection::Down => {
                head_position._y -= 1;
            }
        };

        if head_position._x < 0 || head_position._y < 0 || head_position._x as u32 >= ARENA_WIDTH || head_position._y as u32 >= ARENA_HEIGHT{
            snake_dies_writer.send(SnakeDies);
        }

        if body_segment_positions.contains(&head_position){
            snake_dies_writer.send(SnakeDies)
        }

        body_segment_positions
        .iter()
        .zip(segments.iter().skip(1))
        .for_each(|(pos, segment)| {
            *positions.get_mut(*segment).unwrap() = *pos;
        });
    }
}

pub fn snake_die(mut commands: Commands, mut reader: EventReader<SnakeDies>,
    segments_res: ResMut<SnakeBody>, food: Query<Entity, With<Food>>,
    segments: Query<Entity, With<SnakeSegment>>,){

        if reader.iter().next().is_some() {
            for ent in food.iter().chain(segments.iter()) {
                commands.entity(ent).despawn();
            }
            spawn_snake(commands, segments_res);
        }
    }

pub fn eat_food(
    mut commands: Commands,
    mut growth_writer: EventWriter<GrowthEvent>,
    food_positions: Query<(Entity, &Position), With<Food>>,
    head_positions: Query<&Position, With<SnakeHead>>) {

    for head_pos in head_positions.iter() {
        for (ent, food_pos) in food_positions.iter() {
            if food_pos == head_pos {
                commands.entity(ent).despawn();
                growth_writer.send(GrowthEvent);
            }
        }
    }
}

pub fn snake_growth(commands: Commands, last_body_segment_position: Res<LastBodySegmentPosition>, 
    mut segments: ResMut<SnakeBody>, 
    mut growth_reader: EventReader<GrowthEvent>){

        if growth_reader.iter().next().is_some(){
            segments.push(spawn_snake_body_segment(commands, last_body_segment_position.0.unwrap()))
        }
    }