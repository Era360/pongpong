use crate::constants::{
    BALL_COLOR, BALL_RADIUS, BALL_SPEED, MAX_BALL_SPEED, MOTION_BLUR_ENABLED,
    MOTION_BLUR_TRAIL_COUNT,
};
use ggez::glam::Vec2;
use ggez::graphics::Color;
use rand::{rng, Rng};
use std::collections::VecDeque;
use std::f32::consts::FRAC_PI_4;

pub struct Ball {
    pub position: Vec2,
    pub velocity: Vec2,
    pub radius: f32,
    pub color: Color,
    pub previous_positions: VecDeque<Vec2>, // Store previous positions for motion blur
    pub speed_multiplier: f32,              // For power-ups and game variants
    pub base_speed: f32,                    // The base speed without multipliers
}

impl Ball {
    pub fn new(position: Vec2) -> Self {
        // Generate random initial velocity using rng
        let mut rng = rng();
        let angle = rng.random_range(-FRAC_PI_4..FRAC_PI_4);
        let direction = if rng.random_bool(0.5) { 1.0 } else { -1.0 };

        let mut previous_positions = VecDeque::with_capacity(MOTION_BLUR_TRAIL_COUNT);
        for _ in 0..MOTION_BLUR_TRAIL_COUNT {
            previous_positions.push_back(position);
        }

        Ball {
            position,
            radius: BALL_RADIUS,
            color: BALL_COLOR,
            velocity: Vec2::new(
                direction * angle.cos() * BALL_SPEED,
                angle.sin() * BALL_SPEED,
            ),
            previous_positions,
            speed_multiplier: 1.0,
            base_speed: BALL_SPEED,
        }
    }

    // Create a new ball as a split from an existing ball (used for multiball power-up)
    pub fn split_from(original: &Ball) -> Self {
        let mut rng = rng();

        // Create a slight variation in angle for the split ball
        let angle_deviation = rng.random_range(-FRAC_PI_4..FRAC_PI_4);
        let current_angle = original.velocity.y.atan2(original.velocity.x);
        let new_angle = current_angle + angle_deviation;

        let speed = original.velocity.length();

        let mut previous_positions = VecDeque::with_capacity(MOTION_BLUR_TRAIL_COUNT);
        for _ in 0..MOTION_BLUR_TRAIL_COUNT {
            previous_positions.push_back(original.position);
        }

        Ball {
            position: original.position,
            radius: original.radius,
            color: original.color,
            velocity: Vec2::new(new_angle.cos() * speed, new_angle.sin() * speed),
            previous_positions,
            speed_multiplier: original.speed_multiplier,
            base_speed: original.base_speed,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        // Store the current position before updating for motion blur
        if MOTION_BLUR_ENABLED {
            self.previous_positions.pop_front();
            self.previous_positions.push_back(self.position);
        }

        // Update the ball's position using substepping for smoother movement
        // This divides the update into smaller steps for better collision detection
        const SUBSTEPS: usize = 3;
        let sub_delta = delta_time / SUBSTEPS as f32;

        for _ in 0..SUBSTEPS {
            self.position += self.velocity * sub_delta;
        }
    }

    pub fn bounce_vertical(&mut self) {
        // Reverse the vertical component of the velocity
        self.velocity.y = -self.velocity.y;
    }

    pub fn bounce_horizontal(&mut self) {
        // Reverse the horizontal component of the velocity
        self.velocity.x = -self.velocity.x;
    }

    pub fn reset(&mut self, position: Vec2) {
        // Reset the ball's position and velocity
        self.position = position;

        // Clear motion blur trail
        if MOTION_BLUR_ENABLED {
            for pos in self.previous_positions.iter_mut() {
                *pos = position;
            }
        }

        let mut rng = rng();
        let angle = rng.random_range(-FRAC_PI_4..FRAC_PI_4);
        let direction = if rng.random_bool(0.5) { 1.0 } else { -1.0 };

        // Reset speed multiplier
        self.speed_multiplier = 1.0;
        self.base_speed = BALL_SPEED;

        self.velocity = Vec2::new(
            direction * angle.cos() * self.base_speed * self.speed_multiplier,
            angle.sin() * self.base_speed * self.speed_multiplier,
        );
    }

    // Apply a speed multiplier from a power-up or game mode
    pub fn apply_speed_multiplier(&mut self, multiplier: f32) {
        let current_speed = self.velocity.length();
        self.speed_multiplier = multiplier;

        // Ensure we don't exceed max speed
        let new_speed = (self.base_speed * self.speed_multiplier).min(MAX_BALL_SPEED);

        // Scale velocity to the new speed
        if current_speed > 0.0 {
            self.velocity = self.velocity.normalize() * new_speed;
        }
    }

    // Accelerate the ball based on game mode
    pub fn accelerate(&mut self, acceleration: f32, delta_time: f32) {
        let current_speed = self.velocity.length();
        let speed_increase = acceleration * delta_time;

        // Calculate new speed, capped at maximum
        let new_speed = (current_speed + speed_increase).min(MAX_BALL_SPEED);

        // Only change if there's a meaningful difference
        if (new_speed - current_speed).abs() > 0.1 && current_speed > 0.0 {
            self.velocity = self.velocity.normalize() * new_speed;
        }
    }

    // Normalize the velocity to maintain consistent speed
    pub fn normalize_velocity(&mut self) {
        let speed = self.velocity.length();
        let target_speed = self.base_speed * self.speed_multiplier;

        if speed > 0.0 && (speed < target_speed * 0.9 || speed > target_speed * 1.1) {
            self.velocity = self.velocity.normalize() * target_speed;
        }
    }

    // Get the motion blur positions for rendering
    pub fn get_motion_blur_positions(&self) -> &VecDeque<Vec2> {
        &self.previous_positions
    }
}
