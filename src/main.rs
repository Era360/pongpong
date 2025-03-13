mod constants;
mod entities {
    pub mod ball;
    pub mod direction;
    pub mod player;
}

use ggez::{
    conf::{WindowMode, WindowSetup},
    event,
    glam::*,
    graphics::{self, Color, Rect},
    input::keyboard::{KeyCode, KeyInput},
    Context, ContextBuilder, GameResult,
};

// Constants
use crate::entities::{ball::Ball, direction::Direction, player::Player};
use constants::{PLAYER_PADDING, PLAYER_SIZE, SCREEN_SIZE};

// Main game state structure.
struct GameState {
    yanga_player: Player,
    simba_player: Player,
    ball: Ball,
    left_direction: Option<Direction>,
    right_direction: Option<Direction>,
    yanga_score: u32,
    simba_score: u32,
}

impl GameState {
    // Creates a new game state.
    fn new(_ctx: &mut Context) -> GameResult<GameState> {
        let yanga_player_mesh = Player::new(
            Color::new(0.20, 0.64, 0.31, 1.0),
            Vec2::new(PLAYER_PADDING, SCREEN_SIZE.1 / 2.0 - PLAYER_SIZE.1 / 2.0),
        );

        let simba_player_mesh = Player::new(
            Color::new(0.74, 0.13, 0.19, 1.0),
            Vec2::new(
                SCREEN_SIZE.0 - PLAYER_SIZE.0 - PLAYER_PADDING,
                SCREEN_SIZE.1 / 2.0 - PLAYER_SIZE.1 / 2.0,
            ),
        );

        Ok(GameState {
            yanga_player: yanga_player_mesh,
            simba_player: simba_player_mesh,
            ball: Ball::new(Vec2::new(SCREEN_SIZE.0 / 2.0, SCREEN_SIZE.1 / 2.0)),
            left_direction: None,
            right_direction: None,
            yanga_score: 0,
            simba_score: 0,
        })
    }
}

impl event::EventHandler<ggez::GameError> for GameState {
    // Updates the game state.
    fn update(&mut self, _ctx: &mut Context) -> GameResult {

        // Update positions of the players based on button press
        self.yanga_player.update(self.left_direction);
        self.simba_player.update(self.right_direction);

        // Update the ball's position
        self.ball.update(0.016); // Assuming a fixed delta time for simplicity

        // Check for collisions with the players
        if self.ball.position.x - self.ball.radius < self.yanga_player.position.x + PLAYER_SIZE.0
            && self.ball.position.y > self.yanga_player.position.y
            && self.ball.position.y < self.yanga_player.position.y + PLAYER_SIZE.1
        {
            // Ball hit the left player
            self.ball.position.x = self.yanga_player.position.x + PLAYER_SIZE.0 + self.ball.radius;
        } else if self.ball.position.x + self.ball.radius > self.simba_player.position.x
            && self.ball.position.y > self.simba_player.position.y
            && self.ball.position.y < self.simba_player.position.y + PLAYER_SIZE.1
        {
            // Ball hit the right player
            self.ball.position.x = self.simba_player.position.x - self.ball.radius;
        }

        Ok(())
    }

    // Draws the game state.
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::WHITE);

        // Draw scores on both sides of the players
        // let yanga_score_text = graphics::Text::new(format!("{}", self.yanga_score));
        // let simba_score_text = graphics::Text::new(format!("{}", self.simba_score));
        // let yanga_score_position = Vec2::new(
        //     self.yanga_player.position.x + PLAYER_SIZE.0 / 2.0 - yanga_score_text.width(ctx) as f32 / 2.0,
        //     self.yanga_player.position.y - 20.0,
        // );

        let yanga_player = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            Rect::new(
                self.yanga_player.position.x,
                self.yanga_player.position.y,
                self.yanga_player.size.x,
                self.yanga_player.size.y,
            ),
            self.yanga_player.color,
        )?;

        let simba_player = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            Rect::new(
                self.simba_player.position.x,
                self.simba_player.position.y,
                self.simba_player.size.x,
                self.simba_player.size.y,
            ),
            self.simba_player.color,
        )?;

        // draw the rectangles on both ends of the screens
        canvas.draw(&yanga_player, graphics::DrawParam::default());
        canvas.draw(&simba_player, graphics::DrawParam::default());

        // Draw the ball
        let ball = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            self.ball.position,
            self.ball.radius,
            0.1,
            self.ball.color,
        )?;

        canvas.draw(&ball, graphics::DrawParam::default());

        canvas.finish(ctx)?;

        Ok(())
    }

    // Handles key press events.
    fn key_down_event(&mut self, _ctx: &mut Context, input: KeyInput, _repeat: bool) -> GameResult {
        if let Some(key_code) = input.keycode {
            match key_code {
                KeyCode::W => {
                    self.left_direction = Some(Direction::Up);
                }
                KeyCode::S => {
                    self.left_direction = Some(Direction::Down);
                }
                KeyCode::Up => {
                    self.right_direction = Some(Direction::Up);
                }
                KeyCode::Down => {
                    self.right_direction = Some(Direction::Down);
                }
                _ => {}
            }
        }

        Ok(())
    }

    // Handles key release events.
    fn key_up_event(&mut self, _ctx: &mut Context, input: KeyInput) -> GameResult {
        if let Some(key_code) = input.keycode {
            match key_code {
                KeyCode::W | KeyCode::S => {
                    self.left_direction = None;
                }
                KeyCode::Up | KeyCode::Down => {
                    self.right_direction = None;
                }
                _ => {}
            }
        }

        Ok(())
    }
}

/// Main function to run the game.
pub fn main() -> GameResult {
    // Make a Context.
    let (mut ctx, event_loop) = ContextBuilder::new("pong_pong", "era_eric")
        .window_setup(WindowSetup::default().title("Pong Pong"))
        .window_mode(WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1))
        .build()
        .expect("Could not create ggez context!");

    // Creating an instance of event handler.
    let state = GameState::new(&mut ctx)?;

    // Running the game loop.
    event::run(ctx, event_loop, state)
}
