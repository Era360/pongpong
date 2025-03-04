mod constants;
mod entities {
    pub mod direction;
    pub mod player;
}

use entities::player::Player;
use ggez::graphics::Rect;
use ggez::input::keyboard::{KeyCode, KeyInput};
use ggez::{
    event,
    glam::*,
    graphics::{self, Color},
    Context, ContextBuilder, GameResult,
};
// Constants
use crate::entities::direction::Direction;
use constants::{PLAYER_PADDING, PLAYER_SIZE, SCREEN_SIZE};

// Main game state structure.
struct GameState {
    yanga_player: Player,
    simba_player: Player,
    left_direction: Option<Direction>,
    right_direction: Option<Direction>,
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
            left_direction: None,
            right_direction: None,
        })
    }
}

impl event::EventHandler<ggez::GameError> for GameState {
    // Updates the game state.
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.yanga_player.update(self.left_direction);
        self.simba_player.update(self.right_direction);

        Ok(())
    }

    // Draws the game state.
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::WHITE);

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
        .build()
        .expect("Could not create ggez context!");

    // Creating an instance of event handler.
    let state = GameState::new(&mut ctx)?;

    // Running the game loop.
    event::run(ctx, event_loop, state)
}
