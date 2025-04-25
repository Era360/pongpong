use ggez::{glam::*, graphics::Color};
// use std::time::Duration;

// constants
use crate::constants::{PLAYER_SIZE, PLAYER_SPEED, SCREEN_SIZE};
use crate::entities::direction::Direction;

pub struct Player {
    pub position: Vec2,
    pub size: Vec2,
    pub color: Color,
    pub speed_multiplier: f32,
    pub size_multiplier: f32,
    pub original_size: Vec2,
}

impl Player {
    pub fn new(color: Color, position: Vec2) -> Self {
        Self {
            color,
            position,
            size: Vec2::new(PLAYER_SIZE.0, PLAYER_SIZE.1),
            original_size: Vec2::new(PLAYER_SIZE.0, PLAYER_SIZE.1),
            speed_multiplier: 1.0,
            size_multiplier: 1.0,
        }
    }

    pub fn update(&mut self, direction: Option<Direction>, delta_time: f32) {
        if let Some(direction) = direction {
            match direction {
                Direction::Up => {
                    if self.position.y > 0.0 {
                        self.position.y -= PLAYER_SPEED * self.speed_multiplier * delta_time;
                    }
                }
                Direction::Down => {
                    if self.position.y + self.size.y < SCREEN_SIZE.1 {
                        self.position.y += PLAYER_SPEED * self.speed_multiplier * delta_time;
                    }
                }
            }
        }
    }

    // Apply a speed multiplier from a power-up
    pub fn apply_speed_multiplier(&mut self, multiplier: f32) {
        self.speed_multiplier = multiplier;
    }

    // Apply a size multiplier from a power-up
    pub fn apply_size_multiplier(&mut self, multiplier: f32) {
        self.size_multiplier = multiplier;
        self.update_size();
    }

    // Update paddle size based on the current multiplier
    fn update_size(&mut self) {
        let new_height = self.original_size.y * self.size_multiplier;

        // Ensure the paddle doesn't exceed screen bounds
        let max_height = SCREEN_SIZE.1 * 0.8;
        let clamped_height = new_height.min(max_height);

        // Update the size
        self.size.y = clamped_height;

        // Make sure the paddle is still in bounds
        if self.position.y + self.size.y > SCREEN_SIZE.1 {
            self.position.y = SCREEN_SIZE.1 - self.size.y;
        }
    }

    // Reset power-up effects
    pub fn reset_power_ups(&mut self) {
        self.speed_multiplier = 1.0;
        self.size_multiplier = 1.0;
        self.update_size();
    }
}
