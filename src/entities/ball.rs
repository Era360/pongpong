use crate::constants::{BALL_COLOR, BALL_RADIUS, BALL_SPEED};
use ggez::glam::Vec2;
use ggez::graphics::Color;

pub struct Ball {
    pub position: Vec2,
    pub radius: f32,
    pub color: Color,
}

impl Ball {
    pub fn new(position: Vec2) -> Self {
        Ball {
            position,
            radius: BALL_RADIUS,
            color: BALL_COLOR,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        // Update the ball's position based on its speed and direction
        self.position += Vec2::new(BALL_SPEED * delta_time, 0.0);
    }
}
