use ggez::{
    event,
    glam::*,
    graphics::{self, Color},
    input::keyboard::KeyCode,
    Context, ContextBuilder, GameResult,
};

// Constants
const SCREEN_SIZE: (f32, f32) = (800.0, 600.0); // Screen dimensions
const DESIRED_FPS: u32 = 20; // Desired frames per second

/// Enum representing the direction of movement.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    /// Converts a `KeyCode` to an optional `Direction`.
    ///
    /// # Arguments
    ///
    /// * `key` - A `KeyCode` representing the key pressed.
    ///
    /// # Returns
    ///
    /// * `Option<Direction>` - The corresponding direction if the key matches, otherwise `None`.
    fn from_keycode(key: KeyCode) -> Option<Self> {
        match key {
            KeyCode::W | KeyCode::Up => Some(Direction::Up),
            KeyCode::S | KeyCode::Down => Some(Direction::Down),
            KeyCode::A | KeyCode::Left => Some(Direction::Left),
            KeyCode::D | KeyCode::Right => Some(Direction::Right),
            _ => None,
        }
    }
}

/// Main game state structure.
struct MainState {
    yanga_rect: graphics::Mesh,
    simba_rect: graphics::Mesh,
}

impl MainState {
    /// Creates a new `MainState` instance.
    ///
    /// # Arguments
    ///
    /// * `ctx` - A mutable reference to the `Context`.
    ///
    /// # Returns
    ///
    /// * `GameResult<MainState>` - The new game state instance.
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let yanga_player_mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::stroke(1.0),
            graphics::Rect::new(450.0, 450.0, 50.0, 50.0),
            Color::new(50.0, 168.0, 82.0, 1.0),
        )
        .expect("failed to create yanga player");

        let simba_player_mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::stroke(1.0),
            graphics::Rect::new(300.0, 300.0, 50.0, 50.0),
            Color::new(163.0, 8.0, 1.0, 1.0),
        )
        .expect("failed to create simba player");

        Ok(MainState {
            yanga_rect: yanga_player_mesh,
            simba_rect: simba_player_mesh,
        })
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    /// Updates the game state.
    ///
    /// # Arguments
    ///
    /// * `_ctx` - A mutable reference to the `Context`.
    ///
    /// # Returns
    ///
    /// * `GameResult` - The result of the update operation.
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    /// Draws the game state.
    ///
    /// # Arguments
    ///
    /// * `ctx` - A mutable reference to the `Context`.
    ///
    /// # Returns
    ///
    /// * `GameResult` - The result of the draw operation.
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::WHITE);

        canvas.finish(ctx)?;

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
    let state = MainState::new(&mut ctx)?;

    // Running the game loop.
    event::run(ctx, event_loop, state)
}
