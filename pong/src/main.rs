use std::f32::consts::PI;

use ball::{Ball, MoveResult};
use macroquad::{color::{BLACK, WHITE}, input::{is_key_down, KeyCode}, text::draw_text, time::get_frame_time, window::{clear_background, next_frame, request_new_screen_size}};
use player::{Player, PADDLE_WIDTH};
use screen_controls::{GameBorder, Position};

mod player;
mod screen_controls;
mod ball;

pub const SCREEN_HEIGHT: f32 = 1600.0;
pub const SCREEN_WIDTH : f32 = 1600.0;
pub const SCREEN_CENTER_X : f32 = SCREEN_WIDTH / 2.0;

#[derive(PartialEq, Eq)]
enum GameStatus {
    Playing,
    AwaitPoint,
    GameOver,
}

struct Game {

    border: GameBorder,

    p1: Player,
    p2: Player,
    p1_turn: bool,

    ball: Ball,
    last_impact: Option<f32>,
}

impl Game {
    pub fn new() -> Self {
        Self {
            border: GameBorder::new(),

            p1:  Player::new(
                true,
                Position::new(0.0, SCREEN_HEIGHT / 2.0)
            ),
            p2:  Player::new(
                false,
                Position::new(SCREEN_WIDTH - PADDLE_WIDTH, SCREEN_HEIGHT / 2.0)
            ),
            p1_turn: true,

            ball: Ball::new(
                Position::new(SCREEN_WIDTH / 2.0, SCREEN_HEIGHT / 2.0),
            ),
            last_impact: None,
        }
    }

    pub fn active_player(&mut self) -> &mut Player {
        if self.p1_turn {
            &mut self.p1
        } else {
            &mut self.p2
        }
    }

    pub fn tick_game(&mut self, frame_time: f32) -> GameStatus {
        let (player, opponent) = if self.p1_turn {
            (&mut self.p1, &mut self.p2)
        } else {
            (&mut self.p2, &mut self.p1)
        };

        let move_result = self.ball.try_move(frame_time, self.border.y_constraints());

        if move_result == MoveResult::OutOfBounds {
            opponent.score_point();
            return GameStatus::AwaitPoint
        }


        if is_key_down(KeyCode::W) {
            player.move_up(frame_time, self.border.y_min());
        } else if is_key_down(KeyCode::S) {
            player.move_down(frame_time, self.border.y_max());
        }

        if let Some(normalized_impact) = player.check_impact(&self.ball) {
            self.last_impact = Some(normalized_impact);
            self.ball.react(normalized_impact);
            self.p1_turn = !self.p1_turn;
        }

        GameStatus::Playing
    }

    pub fn debug_impact(&self) {
        match self.last_impact {
            None => {
                draw_text(
                    &format!("Norm Impact: None"), 
                    50.0, 200.0, 
                    30.0, WHITE
                );
            }
            Some(normalized_impact) => {
                draw_text(
                    &format!("Norm Impact: {}, Sin = {}", normalized_impact, (normalized_impact * PI / 4.0).sin()), 
                    50.0, 200.0, 
                    30.0, WHITE
                );
            }
        }
    }

    pub fn draw_game(&self) {
        self.p1.draw();
        self.p2.draw();

        self.ball.draw();
    }

    pub fn reset_for_point(&mut self) {

        self.p1.set_position(Position::new(0.0, SCREEN_HEIGHT / 2.0));
        self.p2.set_position(Position::new(SCREEN_WIDTH - PADDLE_WIDTH, SCREEN_HEIGHT / 2.0));

        self.ball.restore_for_point(
            Position::new(SCREEN_WIDTH / 2.0, SCREEN_HEIGHT / 2.0)
        );
    }

    pub fn start_play(&mut self) {

        self.ball.init_random_velocity();

        // Allow the right player to start the move
        self.p1_turn = self.ball.moving_left();
    }
}

#[macroquad::main("Pong")]
async fn main() {
    println!("Hello, world!");
    
    request_new_screen_size(SCREEN_WIDTH, SCREEN_HEIGHT);

    let mut game = Game::new();

    // Frame Rate is 144 HZ so frame time is 6.9 millis

    let mut frame_time: f32;
    let mut game_status: GameStatus = GameStatus::AwaitPoint;

    loop {
        clear_background(BLACK);


        if game_status == GameStatus::AwaitPoint {

            if is_key_down(KeyCode::Space) {
                game.start_play();
                game_status = GameStatus::Playing;
            }
            
        } else {

            frame_time = get_frame_time();

            game_status = game.tick_game(frame_time);

            if game_status == GameStatus::AwaitPoint {

                game.reset_for_point();
            }

        }

        game.border.draw_border();
        game.draw_game();


        next_frame().await
    }
}
