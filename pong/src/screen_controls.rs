use std::time::{SystemTime, UNIX_EPOCH};

use macroquad::{color::{GRAY, WHITE}, math::Vec2, shapes::{draw_line, draw_rectangle}};

use crate::{SCREEN_CENTER_X, SCREEN_HEIGHT, SCREEN_WIDTH};


/// Screen Position
/// Define a screen position.
/// X is horizontal and increases from 0 left to right
/// Y is vertical and increases from top to bottom
/// 
pub type Position = Vec2;
pub type Velocity = Vec2;

pub type SizeConstraints = (f32, f32);

const BORDER_SIZE: f32 = 50.0;

const CENTERLINE_WIDTH: f32 = 40.0;

pub struct GameBorder {
    bottom_y_start: f32,
}

impl GameBorder {
    pub fn new() -> Self {

        Self {
            bottom_y_start: SCREEN_HEIGHT - BORDER_SIZE,
        }
    }

    pub fn draw_border(&self) {

        // Draw Top Rect
        draw_rectangle(
            0.0, 0.0, 
            SCREEN_WIDTH, BORDER_SIZE, 
            GRAY
        );

        // Draw BOTTOM Rect
        draw_rectangle(
            0.0, self.bottom_y_start, 
            SCREEN_WIDTH, BORDER_SIZE, 
            GRAY
        );

        let available_y = self.bottom_y_start - BORDER_SIZE;
        // let centerline_x = SCREEN_CENTER_X - CENTERLINE_WIDTH / 2.0;
        let centerline_x = SCREEN_CENTER_X;
        let segment_height = available_y / 30.0;
        let mut start_y;
        //Centerline
        for i in (0 .. 30).step_by(2) {
            start_y = BORDER_SIZE + segment_height / 2.0 + segment_height * i as f32;
            draw_line(
                centerline_x, start_y, 
                centerline_x, start_y + segment_height, 
                CENTERLINE_WIDTH, 
                WHITE
            );
        }
    }

    pub fn y_min(&self) -> f32 {
        BORDER_SIZE
    }
    pub fn y_max(&self) -> f32 {
        self.bottom_y_start
    }
    pub fn y_constraints(&self) -> SizeConstraints {
        (BORDER_SIZE, self.bottom_y_start)
    }
}

#[allow(unused)]
pub fn vec2_magnitude(v: Vec2) -> f32 {
    (v.x.powi(2) + v.y.powi(2)).sqrt()
}


pub fn get_system_time_seconds() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
}