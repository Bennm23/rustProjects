use std::f32::consts::PI;

use macroquad::{color::WHITE, rand::{gen_range, srand}, shapes::draw_circle, text::draw_text};

use crate::{screen_controls::{get_system_time_seconds, Position, SizeConstraints, Velocity}, SCREEN_WIDTH};

const BASE_BALL_SPEED_PPS  : f32 = 700.0;
const BALL_SPEED_MULTIPLIER: f32 = 1.15;


const BALL_RADIUS: f32 = 30.0;
const BALL_MAX_WIDTH : f32 = SCREEN_WIDTH - BALL_RADIUS;

pub struct Ball {
    position: Position,
    velocity: Position,
    ball_speed: f32, // Ball speed, in pixels per second
}

#[derive(PartialEq, Eq)]
pub enum MoveResult {
    OutOfBounds,
    InBounds,
}

impl Ball {

    pub fn new(position: Position) -> Self {

        Self {
            position,
            velocity: Velocity::new(-BASE_BALL_SPEED_PPS, 0.0),
            ball_speed: BASE_BALL_SPEED_PPS,
        }
    }

    pub fn restore_for_point(&mut self, position: Position) {
        self.position = position;
        self.ball_speed = BASE_BALL_SPEED_PPS;
    }
    pub fn init_random_velocity(&mut self) {
        srand(get_system_time_seconds());
        let rand_y: f32 = gen_range(-0.5, 0.5);

        self.velocity.y = self.ball_speed * rand_y;

        let positive: f32 = gen_range(0.0, 1.0);
        // Remaining X velocity
        let mut x_velocity = (self.ball_speed.powi(2) - self.velocity.y.powi(2)).sqrt();

        if positive >= 0.5 {
            x_velocity *= -1.0;
        }
        self.velocity.x = x_velocity;
    }

    pub fn moving_left(&self) -> bool {
        self.velocity.x < 0.0
    }

    pub fn try_move(&mut self, frame_time: f32, y_constraints: SizeConstraints) -> MoveResult {
        self.position += self.velocity * frame_time;

    
        // Check if paddle missed so point over
        if self.position.x <= BALL_RADIUS || self.position.x >= BALL_MAX_WIDTH {
            self.velocity.x *= -1.0;
            return MoveResult::OutOfBounds;
        }

        // Check for bounce on top/bottom
        if self.position.y - BALL_RADIUS <= y_constraints.0 ||
            self.position.y + BALL_RADIUS >= y_constraints.1        
        {
            self.velocity.y *= -1.0;
        }
        MoveResult::InBounds
    }

    pub fn draw(&self) {
        draw_circle(
            self.position.x, 
            self.position.y, 
            BALL_RADIUS, 
            WHITE,
        );
    }

    #[allow(unused)]
    pub fn debug_position(&self) {
        draw_text(
            &format!("Ball Pos: X = {}, Y = {}", self.position.x, self.position.y), 
            100.0, 100.0, 
            30.0, WHITE
        );
    }

    #[allow(unused)]
    pub fn debug_velocity(&self) {
        draw_text(
            &format!("Ball Velo: X = {}, Y = {}", self.velocity.x, self.velocity.y), 
            800.0, 100.0, 
            30.0, WHITE
        );

    }

    pub fn get_exterior_position(&self) -> Position {
        // If moving left, check left side of ball
        if self.velocity.x < 0.0 {
            Position::new(self.position.x - BALL_RADIUS, self.position.y)
        } else {
            Position::new(self.position.x + BALL_RADIUS, self.position.y)
        }
    }

    /// React
    /// Given an impact value from [-1, 1],
    /// adjust our current velocity
    /// normalized_impact > 0 when impacting the bottom 
    ///     half of the paddle
    pub fn react(&mut self, normalized_impact: f32) {
        let now_moving_left = if self.velocity.x >= 0.0 {
            true
        } else {
            false
        };
        // Increase Speed on impact by 5%
        self.ball_speed *= BALL_SPEED_MULTIPLIER;

        // Sin function that oscillates ranges from [-PI/4, PI/4] = [-0.7, 0.7]
        // over the impact domain. 
        // This allows us to transfer velocity appropriately
        let sin_result = (normalized_impact * PI / 4.0).sin();

        // When result is positive, we impacted the bottom.
        // So we want to move down
        self.velocity.y = -1.0 * sin_result * self.ball_speed;

        self.velocity.x = (self.ball_speed.powi(2) - self.velocity.y.powi(2)).sqrt();

        if now_moving_left {
            self.velocity.x *= -1.0;
        }
    }
}

impl Default for Ball {
    fn default() -> Self {
        Self { 
           position: Default::default(),
           velocity: Default::default(),
           ball_speed: Default::default(),
        }
    }
}