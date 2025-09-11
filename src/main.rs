mod constants;
mod entities {
    pub mod ball;
    pub mod direction;
    pub mod player;
}
mod effects;
mod game_mode;
mod powerups;

use ggez::graphics::Drawable;
use ggez::{
    conf::{WindowMode, WindowSetup},
    event,
    glam::*,
    graphics::{self, Color, Rect, Text, TextFragment},
    input::keyboard::{KeyCode, KeyInput},
    Context, ContextBuilder, GameResult,
};
// Constants
use crate::constants::{
    BALL_ACCELERATION_FACTOR, BALL_SPEED, CENTER_LINE_COLOR, CENTER_LINE_DASH_LENGTH,
    CENTER_LINE_GAP_LENGTH, CENTER_LINE_WIDTH, COUNTDOWN_SECONDS, LONG_RALLY_SPEED_MULTIPLIER,
    LONG_RALLY_THRESHOLD, PADDLE_HIT_PARTICLE_COUNT, PARTICLES_ENABLED, POWERUPS_ENABLED,
    SCREEN_SHAKE_DURATION, SCREEN_SHAKE_ENABLED, SCREEN_SHAKE_INTENSITY, WALL_HIT_PARTICLE_COUNT,
};
use crate::effects::{countdown::Countdown, particles::ParticleSystem};
use crate::entities::{ball::Ball, direction::Direction, player::Player};
use crate::game_mode::GameMode;
use crate::powerups::manager::PowerUpManager;
use constants::{PLAYER_PADDING, PLAYER_SIZE, SCREEN_SIZE};
use rand::Rng;
use std::time::Duration;

// Main game state structure.
struct GameState {
    yanga_player: Player,
    simba_player: Player,
    balls: Vec<Ball>, // Now we support multiple balls
    left_direction: Option<Direction>,
    right_direction: Option<Direction>,
    yanga_score: u32,
    simba_score: u32,
    // Visual effects
    particle_system: ParticleSystem,
    countdown: Countdown,
    screen_shake: Option<(f32, Duration)>, // (intensity, remaining duration)
    game_paused: bool,
    round_in_progress: bool,
    // Power-ups
    power_up_manager: PowerUpManager,
    // Game variants
    game_mode: GameMode,
    rally_count: i32, // For RallyFever mode
    game_time: f32,   // Total game time for Accelerating mode
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

        let mut state = GameState {
            yanga_player: yanga_player_mesh,
            simba_player: simba_player_mesh,
            balls: vec![Ball::new(Vec2::new(
                SCREEN_SIZE.0 / 2.0,
                SCREEN_SIZE.1 / 2.0,
            ))],
            left_direction: None,
            right_direction: None,
            yanga_score: 0,
            simba_score: 0,
            // Visual effects
            particle_system: ParticleSystem::new(constants::MAX_PARTICLES),
            countdown: Countdown::new(COUNTDOWN_SECONDS),
            screen_shake: None,
            game_paused: false,
            round_in_progress: false,
            // Power-ups
            power_up_manager: PowerUpManager::new(),
            // Game variants
            game_mode: GameMode::Classic,
            rally_count: 0,
            game_time: 0.0,
        };

        // Start with a countdown
        state.start_new_round();

        Ok(state)
    }

    // Starts a new round with countdown
    fn start_new_round(&mut self) {
        self.game_paused = true;
        self.round_in_progress = false;
        self.countdown.start();

        // Reset players and power-ups
        self.yanga_player.reset_power_ups();
        self.simba_player.reset_power_ups();
        self.power_up_manager.reset();

        // Reset balls
        self.balls.clear();
        self.balls.push(Ball::new(Vec2::new(
            SCREEN_SIZE.0 / 2.0,
            SCREEN_SIZE.1 / 2.0,
        )));

        // Reset game mode specific counters
        self.rally_count = 0;
    }

    // Adds screen shake effect
    fn add_screen_shake(&mut self, intensity: f32) {
        if SCREEN_SHAKE_ENABLED {
            self.screen_shake = Some((intensity, Duration::from_secs_f32(SCREEN_SHAKE_DURATION)));
        }
    }

    // Cycles to the next game mode
    fn cycle_game_mode(&mut self) {
        self.game_mode = self.game_mode.next();
        self.start_new_round(); // Reset the game for the new mode
    }

    // Adds a new ball for multiball power-up
    fn add_multiball(&mut self) {
        if self.balls.is_empty() {
            // If no balls exist (shouldn't happen, but just in case)
            self.balls.push(Ball::new(Vec2::new(
                SCREEN_SIZE.0 / 2.0,
                SCREEN_SIZE.1 / 2.0,
            )));
        } else {
            // Create a split from the first ball
            let new_ball = Ball::split_from(&self.balls[0]);
            self.balls.push(new_ball);
        }
    }
}

impl event::EventHandler<ggez::GameError> for GameState {
    // Updates the game state.
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        // Get the actual delta time for smooth movement, with a cap to prevent physics issues
        let mut delta = ctx.time.delta().as_secs_f32();
        if delta > constants::MAX_DELTA_TIME {
            delta = constants::MAX_DELTA_TIME;
        }

        // Update total game time
        self.game_time += delta;

        // Update countdown
        if self.countdown.active {
            if self.countdown.update(ctx.time.delta()) {
                // Countdown finished
                self.game_paused = false;
                self.round_in_progress = true;
            }
        }

        // Update particles
        if PARTICLES_ENABLED {
            self.particle_system.update(ctx.time.delta());
        }

        // Update screen shake
        if let Some((_intensity, remaining)) = &mut self.screen_shake {
            let new_remaining = remaining.saturating_sub(ctx.time.delta());
            if new_remaining.as_millis() == 0 {
                self.screen_shake = None;
            } else {
                *remaining = new_remaining;
            }
        }

        // Skip game logic updates if game is paused
        if self.game_paused {
            return Ok(());
        }

        // Update positions of the players based on button press
        self.yanga_player.update(self.left_direction, delta);
        self.simba_player.update(self.right_direction, delta);

        // Update power-ups if enabled
        if POWERUPS_ENABLED && !self.balls.is_empty() {
            self.power_up_manager.update(
                ctx.time.delta(),
                &self.balls[0],
                &mut self.yanga_player,
                &mut self.simba_player,
            );

            // Check if we need to add a multiball
            if self.power_up_manager.get_active_multiball() && self.balls.len() == 1 {
                self.add_multiball();
            }
        }

        // Keep track of screen shake requests
        let mut should_shake = false;

        // Update each ball
        let mut ball_scored = false;
        let mut scoring_player = 0;

        for ball in &mut self.balls {
            // Update the ball's position using actual delta time
            ball.update(delta);

            // Apply game mode specific logic
            match self.game_mode {
                GameMode::Accelerating => {
                    // Gradually accelerate the ball
                    ball.accelerate(BALL_ACCELERATION_FACTOR, delta);
                }
                GameMode::RallyFever => {
                    // Ball speed increases after a long rally
                    if self.rally_count >= LONG_RALLY_THRESHOLD {
                        ball.apply_speed_multiplier(LONG_RALLY_SPEED_MULTIPLIER);
                    }
                }
                GameMode::Classic => {
                    // Standard behavior, just normalize the velocity
                    ball.normalize_velocity();
                }
            }

            // Check for collisions with top and bottom walls
            if ball.position.y - ball.radius <= 0.0
                || ball.position.y + ball.radius >= SCREEN_SIZE.1
            {
                ball.bounce_vertical();

                // Add wall hit particles
                if PARTICLES_ENABLED {
                    let wall_position = if ball.position.y - ball.radius <= 0.0 {
                        Vec2::new(ball.position.x, 0.0)
                    } else {
                        Vec2::new(ball.position.x, SCREEN_SIZE.1)
                    };

                    self.particle_system.emit(
                        wall_position,
                        Color::new(0.8, 0.8, 0.8, 1.0),
                        WALL_HIT_PARTICLE_COUNT,
                    );
                }
            }

            // Check for collisions with the players
            if ball.position.x - ball.radius <= self.yanga_player.position.x + PLAYER_SIZE.0
                && ball.position.y + ball.radius >= self.yanga_player.position.y
                && ball.position.y - ball.radius
                    <= self.yanga_player.position.y + self.yanga_player.size.y
                && ball.velocity.x < 0.0
            // Only bounce if moving toward the player
            {
                // Ball hit the left player
                self.rally_count += 1;

                let paddle_center = self.yanga_player.position.y + self.yanga_player.size.y / 2.0;
                let distance_from_center = ball.position.y - paddle_center;
                let normalized_distance = distance_from_center / (self.yanga_player.size.y / 2.0);

                // Adjust the ball's velocity based on the collision angle
                ball.velocity.x = ball.velocity.x.abs(); // Ensure it moves right
                ball.velocity.y += normalized_distance * BALL_SPEED * 0.5; // Add spin effect

                // Add particles at collision point
                if PARTICLES_ENABLED {
                    self.particle_system.emit(
                        Vec2::new(
                            self.yanga_player.position.x + PLAYER_SIZE.0,
                            ball.position.y,
                        ),
                        self.yanga_player.color,
                        PADDLE_HIT_PARTICLE_COUNT,
                    );
                }

                // Mark for screen shake instead of calling directly
                should_shake = true;
            } else if ball.position.x + ball.radius >= self.simba_player.position.x
                && ball.position.y + ball.radius >= self.simba_player.position.y
                && ball.position.y - ball.radius
                    <= self.simba_player.position.y + self.simba_player.size.y
                && ball.velocity.x > 0.0
            // Only bounce if moving toward the player
            {
                // Ball hit the right player
                self.rally_count += 1;

                let paddle_center = self.simba_player.position.y + self.simba_player.size.y / 2.0;
                let distance_from_center = ball.position.y - paddle_center;
                let normalized_distance = distance_from_center / (self.simba_player.size.y / 2.0);

                // Adjust the ball's velocity based on the collision angle
                ball.velocity.x = -ball.velocity.x.abs(); // Ensure it moves left
                ball.velocity.y += normalized_distance * BALL_SPEED * 0.5; // Add spin effect

                // Add particles at collision point
                if PARTICLES_ENABLED {
                    self.particle_system.emit(
                        Vec2::new(self.simba_player.position.x, ball.position.y),
                        self.simba_player.color,
                        PADDLE_HIT_PARTICLE_COUNT,
                    );
                }

                // Mark for screen shake instead of calling directly
                should_shake = true;
            }

            // Check if the ball went out of bounds (scoring)
            if ball.position.x + ball.radius < 0.0 {
                // Simba scores
                ball_scored = true;
                scoring_player = 1;
                break;
            } else if ball.position.x - ball.radius > SCREEN_SIZE.0 {
                // Yanga scores
                ball_scored = true;
                scoring_player = 0;
                break;
            }
        }

        // Apply screen shake after the loop if needed
        if should_shake {
            self.add_screen_shake(SCREEN_SHAKE_INTENSITY);
        }

        // Handle scoring
        if ball_scored {
            if scoring_player == 0 {
                self.yanga_score += 1;
            } else {
                self.simba_score += 1;
            }
            self.start_new_round();
        }

        Ok(())
    }

    // Draws the game state.
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::WHITE);

        // Apply screen shake if active
        if let Some((intensity, remaining)) = &self.screen_shake {
            if SCREEN_SHAKE_ENABLED {
                let shake_ratio = remaining.as_secs_f32() / SCREEN_SHAKE_DURATION;
                let mut rng = rand::rng();

                let screen_offset = Vec2::new(
                    rng.random_range(-*intensity..*intensity) * shake_ratio,
                    rng.random_range(-*intensity..*intensity) * shake_ratio,
                );

                canvas.set_screen_coordinates(Rect::new(
                    screen_offset.x,
                    screen_offset.y,
                    SCREEN_SIZE.0,
                    SCREEN_SIZE.1,
                ));
            }
        }

        // Draw scores on both sides of the players
        let vs_text = graphics::Text::new(
            graphics::TextFragment::new("VS")
                .color(Color::BLACK)
                .scale(graphics::PxScale::from(15.0)),
        );
        let yanga_score_text = graphics::Text::new(
            graphics::TextFragment::new(format!("{}", self.yanga_score))
                .color(Color::BLACK)
                .scale(graphics::PxScale::from(30.0)),
        );
        let simba_score_text = graphics::Text::new(
            graphics::TextFragment::new(format!("{}", self.simba_score))
                .color(Color::BLACK)
                .scale(graphics::PxScale::from(30.0)),
        );

        // Add the scores at the top of the screen with dramatic VS text
        let yanga_score_position = Vec2::new(
            SCREEN_SIZE.0 / 2.0
                - yanga_score_text.dimensions(ctx).unwrap().w
                - vs_text.dimensions(ctx).unwrap().w
                - constants::TEXT_PADDING,
            PLAYER_PADDING,
        );
        let simba_score_position = Vec2::new(
            SCREEN_SIZE.0 / 2.0 + vs_text.dimensions(ctx).unwrap().w + constants::TEXT_PADDING,
            PLAYER_PADDING,
        );
        let vs_position = Vec2::new(
            SCREEN_SIZE.0 / 2.0 - constants::TEXT_PADDING / 2.0,
            PLAYER_PADDING,
        );

        // Draw the scores and the VS text
        canvas.draw(
            &yanga_score_text,
            graphics::DrawParam::from(yanga_score_position).color(Color::BLACK),
        );
        canvas.draw(
            &simba_score_text,
            graphics::DrawParam::from(simba_score_position).color(Color::BLACK),
        );
        canvas.draw(
            &vs_text,
            graphics::DrawParam::from(vs_position).color(Color::BLACK),
        );

        // Draw the game mode text
        let mode_text = Text::new(
            TextFragment::new(format!("Mode: {}", self.game_mode))
                .color(Color::BLACK)
                .scale(graphics::PxScale::from(20.0)),
        );

        let mode_position = Vec2::new(10.0, SCREEN_SIZE.1 - 30.0);

        canvas.draw(
            &mode_text,
            graphics::DrawParam::from(mode_position).color(Color::BLACK),
        );

        // Create player meshes
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

        // Draw the players
        canvas.draw(&yanga_player, graphics::DrawParam::default());
        canvas.draw(&simba_player, graphics::DrawParam::default());

        // Draw each ball with motion blur
        for ball in &self.balls {
            // Draw motion blur trails if enabled
            if constants::MOTION_BLUR_ENABLED {
                let trail_positions = ball.get_motion_blur_positions();

                // Draw each trail with decreasing opacity
                for (i, &pos) in trail_positions.iter().enumerate() {
                    // Skip the most recent position as it will be drawn as the main ball
                    if i == trail_positions.len() - 1 {
                        continue;
                    }

                    // Calculate opacity based on position in the trail
                    let opacity = 0.2 * (i as f32 / trail_positions.len() as f32);
                    let trail_color = Color::new(ball.color.r, ball.color.g, ball.color.b, opacity);

                    // Draw smaller circles for the trail
                    let trail_radius =
                        ball.radius * (0.7 + 0.3 * (i as f32 / trail_positions.len() as f32));

                    let trail = graphics::Mesh::new_circle(
                        ctx,
                        graphics::DrawMode::fill(),
                        pos,
                        trail_radius,
                        0.1,
                        trail_color,
                    )?;

                    canvas.draw(&trail, graphics::DrawParam::default());
                }
            }

            // Draw the ball
            let ball_mesh = graphics::Mesh::new_circle(
                ctx,
                graphics::DrawMode::fill(),
                ball.position,
                ball.radius,
                0.1, // Reduce the tolerance for better performance
                ball.color,
            )?;

            canvas.draw(&ball_mesh, graphics::DrawParam::default());
        }

        // Draw center line - using dashed line for better visual effect
        let center_x = SCREEN_SIZE.0 / 2.0;
        let mut y = 0.0;

        while y < SCREEN_SIZE.1 {
            let dash_end = (y + CENTER_LINE_DASH_LENGTH).min(SCREEN_SIZE.1);

            let dash = graphics::Mesh::new_line(
                ctx,
                &[Vec2::new(center_x, y), Vec2::new(center_x, dash_end)],
                CENTER_LINE_WIDTH,
                CENTER_LINE_COLOR,
            )?;

            canvas.draw(&dash, graphics::DrawParam::default());

            // Move to the next dash position
            y = dash_end + CENTER_LINE_GAP_LENGTH;
        }

        // Draw power-ups
        if POWERUPS_ENABLED {
            self.power_up_manager.draw(ctx, &mut canvas)?;
        }

        // Draw particle effects
        if PARTICLES_ENABLED {
            self.particle_system.draw(ctx, &mut canvas)?;
        }

        // Draw countdown if active
        if self.countdown.active {
            self.countdown.draw(ctx, &mut canvas)?;
        }

        // Draw paused text if the game is paused and countdown is not active
        if self.game_paused && !self.countdown.active {
            let paused_text = Text::new(
                TextFragment::new("PAUSED") //TODO: maybe we add a nice pause icon?
                    .color(Color::BLACK)
                    .scale(graphics::PxScale::from(50.0)),
            );

            let text_dimensions = paused_text.dimensions(ctx).unwrap();
            let paused_position = Vec2::new(
                SCREEN_SIZE.0 / 2.0 - text_dimensions.w / 2.0,
                SCREEN_SIZE.1 / 2.0 - text_dimensions.h / 2.0,
            );

            canvas.draw(
                &paused_text,
                graphics::DrawParam::from(paused_position).color(Color::BLACK),
            );
        }

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
                KeyCode::Space => {
                    // Toggle pause if space is pressed
                    if !self.countdown.active {
                        self.game_paused = !self.game_paused;
                    }
                }
                KeyCode::R => {
                    // Restart the round if R is pressed
                    self.start_new_round();
                }
                KeyCode::M => {
                    // Switch game mode if M is pressed
                    self.cycle_game_mode();
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
    // Make a Context with VSync enabled for smoother rendering
    let (mut ctx, event_loop) = ContextBuilder::new("pong_pong", "era360")
        .window_setup(WindowSetup::default().title("Pong Pong").vsync(true)) // Enable VSync for smoother rendering
        .window_mode(WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1))
        .build()
        .expect("Could not create ggez context!");

    // Creating an instance of event handler.
    let state = GameState::new(&mut ctx)?;

    // Running the game loop.
    event::run(ctx, event_loop, state)
}
