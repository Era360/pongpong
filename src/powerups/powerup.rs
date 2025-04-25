use crate::constants::{
    POWERUP_DURATION, POWERUP_FLASH_SPEED, POWERUP_FLASH_THRESHOLD, POWERUP_SIZE, SCREEN_SIZE,
};
use ggez::glam::Vec2;
use ggez::graphics::Color;
use ggez::{graphics, Context, GameResult};
use rand::{rng, Rng};
use std::time::Duration;

/// Different types of power-ups that can appear in the game
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PowerUpType {
    PaddleGrow,     // Increases paddle size
    PaddleShrink,   // Decreases opponent's paddle size
    SpeedUp,        // Increases ball speed
    SpeedDown,      // Decreases ball speed
    MultiballSplit, // Splits the ball into two
}

impl PowerUpType {
    /// Returns a random power-up type
    pub fn random() -> Self {
        let mut rng = rng();
        match rng.random_range(0..5) {
            0 => PowerUpType::PaddleGrow,
            1 => PowerUpType::PaddleShrink,
            2 => PowerUpType::SpeedUp,
            3 => PowerUpType::SpeedDown,
            _ => PowerUpType::MultiballSplit,
        }
    }

    /// Returns the color associated with this power-up type
    pub fn color(&self) -> Color {
        match self {
            PowerUpType::PaddleGrow => Color::new(0.0, 0.8, 0.0, 1.0), // Green
            PowerUpType::PaddleShrink => Color::new(0.8, 0.0, 0.0, 1.0), // Red
            PowerUpType::SpeedUp => Color::new(1.0, 0.5, 0.0, 1.0),    // Orange
            PowerUpType::SpeedDown => Color::new(0.0, 0.0, 0.8, 1.0),  // Blue
            PowerUpType::MultiballSplit => Color::new(0.8, 0.0, 0.8, 1.0), // Purple
        }
    }

    // /// Returns a string describing the power-up effect
    // pub fn description(&self) -> &'static str {
    //     match self {
    //         PowerUpType::PaddleGrow => "Paddle Growth",
    //         PowerUpType::PaddleShrink => "Opponent Shrink",
    //         PowerUpType::SpeedUp => "Speed Up",
    //         PowerUpType::SpeedDown => "Speed Down",
    //         PowerUpType::MultiballSplit => "Multiball",
    //     }
    // }
}

/// A power-up that can be collected by either player
pub struct PowerUp {
    pub position: Vec2,
    pub power_type: PowerUpType,
    pub size: f32,
    pub active: bool,
    pub collected_by: Option<usize>, // 0 for left player, 1 for right player
    pub remaining_duration: Option<Duration>,
    pub rotation: f32, // For visual effect
}

impl PowerUp {
    /// Creates a new power-up
    pub fn new(position: Vec2, power_type: PowerUpType) -> Self {
        Self {
            position,
            power_type,
            size: POWERUP_SIZE,
            active: true,
            collected_by: None,
            remaining_duration: None,
            rotation: 0.0,
        }
    }

    /// Creates a new random power-up at a random position
    pub fn random() -> Self {
        let mut rng = rng();

        // Generate a random position (avoid edges and center line)
        let x = if rng.random_bool(0.5) {
            // Left side
            rng.random_range(POWERUP_SIZE * 2.0..SCREEN_SIZE.0 / 2.0 - POWERUP_SIZE * 2.0)
        } else {
            // Right side
            rng.random_range(
                SCREEN_SIZE.0 / 2.0 + POWERUP_SIZE * 2.0..SCREEN_SIZE.0 - POWERUP_SIZE * 2.0,
            )
        };

        let y = rng.random_range(POWERUP_SIZE * 2.0..SCREEN_SIZE.1 - POWERUP_SIZE * 2.0);

        Self::new(Vec2::new(x, y), PowerUpType::random())
    }

    /// Updates the power-up state
    pub fn update(&mut self, delta: Duration) -> bool {
        // Rotate the power-up for visual effect
        self.rotation += 1.0 * delta.as_secs_f32();
        if self.rotation > std::f32::consts::TAU {
            self.rotation -= std::f32::consts::TAU;
        }

        // If the power-up is collected, update its duration
        if let Some(duration) = &mut self.remaining_duration {
            *duration = duration.saturating_sub(delta);

            // Check if the power-up effect has expired
            if duration.as_secs_f32() <= 0.0 {
                return false; // Power-up expired
            }
        }

        true // Power-up still active
    }

    /// Activates the power-up effect when collected
    pub fn activate(&mut self, player_index: usize) {
        self.active = false;
        self.collected_by = Some(player_index);
        self.remaining_duration = Some(Duration::from_secs_f32(POWERUP_DURATION));
    }

    /// Checks if the power-up should flash (about to expire)
    pub fn should_flash(&self) -> bool {
        if let Some(duration) = self.remaining_duration {
            return duration.as_secs_f32() <= POWERUP_FLASH_THRESHOLD;
        }
        false
    }

    /// Gets the current opacity based on flashing state
    pub fn get_opacity(&self, total_time: f32) -> f32 {
        if self.should_flash() {
            // Flash by alternating opacity based on time
            let flash_cycle = (total_time * POWERUP_FLASH_SPEED).sin() * 0.5 + 0.5;
            return 0.5 + flash_cycle * 0.5; // Oscillate between 0.5 and 1.0
        }
        1.0 // Full opacity
    }

    /// Draws the power-up
    pub fn draw(
        &self,
        ctx: &mut Context,
        canvas: &mut graphics::Canvas,
        total_time: f32,
    ) -> GameResult {
        if !self.active {
            return Ok(());
        }

        let mut color = self.power_type.color();
        color.a = self.get_opacity(total_time);

        // Draw a diamond shape for the power-up
        let points = [
            Vec2::new(self.position.x, self.position.y - self.size), // Top
            Vec2::new(self.position.x + self.size, self.position.y), // Right
            Vec2::new(self.position.x, self.position.y + self.size), // Bottom
            Vec2::new(self.position.x - self.size, self.position.y), // Left
        ];

        let mesh = graphics::Mesh::new_polygon(ctx, graphics::DrawMode::fill(), &points, color)?;

        // Apply rotation
        canvas.draw(
            &mesh,
            graphics::DrawParam::default()
                .dest(self.position)
                .offset(Vec2::new(0.5, 0.5))
                .rotation(self.rotation),
        );

        Ok(())
    }

    /// Checks if the power-up collides with the ball
    pub fn collides_with_ball(&self, ball_position: Vec2, ball_radius: f32) -> bool {
        if !self.active {
            return false;
        }

        // Simple circle-circle collision
        let distance = self.position.distance(ball_position);
        distance < (self.size + ball_radius)
    }
}
