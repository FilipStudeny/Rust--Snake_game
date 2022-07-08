

use bevy::prelude::{Component, Color};

pub const ARENA_WIDTH: u32 = 10;
pub const ARENA_HEIGHT: u32 = 10;
pub const ARENA_COLOR: Color = Color::rgb(0.04, 0.04, 0.04);


#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub struct Position{
   pub _x: i32,
   pub _y: i32,
}

#[derive(Component)]
pub struct Size{
    pub _arena_width: f32,
    pub _arena_height: f32,
}

impl Size{
    pub fn square(_x: f32) -> Self{
        Self{ _arena_height: _x, _arena_width: _x }
    }
}