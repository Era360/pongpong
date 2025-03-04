use ggez::{glam::*, graphics::Color};

// constants
use crate::constants::{PLAYER_SIZE, PLAYER_SPEED, SCREEN_SIZE};
use crate::entities::direction::Direction;

pub struct Player {
    pub position: Vec2,
    pub size: Vec2,
    pub color: Color,
}

impl Player {
    pub fn new(color: Color, position: Vec2) -> Self {
        Self {
            color,
            position,
            size: Vec2::new(PLAYER_SIZE.0, PLAYER_SIZE.1),
        }
    }

    pub fn update(&mut self, direction: Option<Direction>) {
        if let Some(direction) = direction {
            match direction {
                Direction::Up => {
                    if self.position.y > 0.0 {
                        self.position.y -= PLAYER_SPEED;
                    }
                }
                Direction::Down => {
                    if self.position.y + self.size.y < SCREEN_SIZE.1 {
                        self.position.y += PLAYER_SPEED;
                    }
                }
            }
        }
    }
}
