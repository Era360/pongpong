use ggez::graphics::Color;

pub const SCREEN_SIZE: (f32, f32) = (800.0, 600.0); // Screen dimensions
pub const PLAYER_SIZE: (f32, f32) = (20.0, SCREEN_SIZE.1 / 4.0); // Player dimensions
pub const PLAYER_SPEED: f32 = 5.0; // Player speed
pub const PLAYER_PADDING: f32 = 5.0; // Player padding

pub const BALL_RADIUS: f32 = 10.0; // Ball radius
pub const BALL_SPEED: f32 = 50.0; // Ball speed
pub const BALL_COLOR: Color = Color::new(0.0, 0.0, 1.0, 1.0); // Ball color

// pub const DESIRED_FPS: u32 = 20; // Desired frames per second
