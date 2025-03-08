use macroquad::{color::{Color, BLUE, RED, WHITE}, shapes::draw_rectangle, text::draw_text};

use crate::{ball::Ball, screen_controls::Position, SCREEN_WIDTH};

pub const PADDLE_WIDTH : f32 =  50.0;
pub const PADDLE_HEIGHT: f32 = 400.0;

const PADDLE_HEIGHT_CORRECTION: f32 = PADDLE_HEIGHT / 2.0;

/// Paddle Speed, vertically, in pixels per second.
const PADDLE_SPEED_PPS: f32 = 500.0;

pub struct Player {
    color: Color,
    center: Position,
    is_left_player: bool,
    score: u16,
}

impl Player {

    pub fn new(left_player: bool, position: Position) -> Self {

        let color = if left_player {
            BLUE
        } else {
            RED
        };
            
        Self {
            color,
            center: position,
            is_left_player: left_player,
            score: 0,
        }
    }

    pub fn set_position(&mut self, position: Position) {
        self.center = position;
    }

    pub fn draw(&self) {
        draw_rectangle(
            self.center.x, 
            self.center.y - PADDLE_HEIGHT_CORRECTION, 
            PADDLE_WIDTH, 
            PADDLE_HEIGHT, 
            self.color,
        );

        if self.is_left_player {
            draw_text(
                &format!("{}", self.score), 
                SCREEN_WIDTH / 2.0 - 150.0, 200.0, 
                200.0, self.color
            );
        } else {
            draw_text(
                &format!("{}", self.score), 
                SCREEN_WIDTH / 2.0 + 65.0, 200.0, 
                200.0, self.color
            );
        }
    }

    fn get_top_y(&self) -> f32 {
        self.center.y - PADDLE_HEIGHT_CORRECTION
    }
    fn get_bottom_y(&self) -> f32 {
        self.center.y + PADDLE_HEIGHT_CORRECTION
    }

    pub fn move_up(&mut self, frame_time: f32, min_val: f32) {
        self.center.y -= frame_time * PADDLE_SPEED_PPS;
        // Constrain to border top
        if self.get_top_y() < min_val {
            self.center.y = min_val + PADDLE_HEIGHT_CORRECTION
        }
    }
    pub fn move_down(&mut self, frame_time: f32, max_val: f32) {
        self.center.y += frame_time * PADDLE_SPEED_PPS;
        // Constrain to border bottom
        if self.get_bottom_y() > max_val {
            self.center.y = max_val - PADDLE_HEIGHT_CORRECTION;
        }
    }

    /// Check Impact
    /// 
    /// Determine whether or not the ball has impacted
    /// this player. Need to consider only side of the ball
    /// that is inbound
    pub fn check_impact(&mut self, ball: &Ball) -> Option<f32> {

        let ball_impact_pos = ball.get_exterior_position();

        // if the ball y is between our position
        // and x is <= or >= our position we have impact
        let hitting_y = ball_impact_pos.y >= self.center.y - PADDLE_HEIGHT_CORRECTION && 
                                ball_impact_pos.y <= self.center.y + PADDLE_HEIGHT_CORRECTION;

        let hitting_x = if self.is_left_player {
            ball_impact_pos.x <= self.center.x + PADDLE_WIDTH
        } else {
            ball_impact_pos.x >= self.center.x
        };

        if hitting_x && hitting_y {
            // Find the impact point along our paddle
            // Our y is the center of the rectangle vertically
            let relative_y = self.center.y - ball_impact_pos.y;

            // < 0 when impacting past midpoint, > 0 when impacting above midpoint
            let normalized_impact = relative_y / PADDLE_HEIGHT_CORRECTION;

            return Some(normalized_impact);
        }
        None
    }

    pub fn score_point(&mut self) {
        self.score += 1;
    }

    #[allow(unused)]
    pub fn draw_info(&self) {
        draw_text(
            &format!("P2: X = {}, Y = {}", self.center.x, self.center.y), 
            100.0, 200.0, 
            30.0, WHITE
        );
    }
}