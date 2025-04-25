use ggez::graphics::Color;

// Screen and game settings
pub const SCREEN_SIZE: (f32, f32) = (1000.0, 700.0); // Screen dimensions
pub const PLAYER_SIZE: (f32, f32) = (20.0, SCREEN_SIZE.1 / 4.0); // Player dimensions
pub const PLAYER_SPEED: f32 = 300.0; // Player speed - adjusted for delta time
pub const PLAYER_PADDING: f32 = 5.0; // Player padding

// Ball settings
pub const BALL_RADIUS: f32 = 10.0; // Ball radius
pub const BALL_SPEED: f32 = 400.0; // Increased ball speed for smoother animation
pub const BALL_COLOR: Color = Color::new(0.0, 0.0, 1.0, 1.0); // Ball color

// Frame rate settings
pub const MAX_DELTA_TIME: f32 = 1.0 / 20.0; // Cap delta time to prevent physics issues
pub const MOTION_BLUR_ENABLED: bool = true; // Enable motion blur for smoother ball movement
pub const MOTION_BLUR_TRAIL_COUNT: usize = 3; // Number of motion blur trails

// Score settings
pub const TEXT_PADDING: f32 = 10.0; // Padding for score text

// Visual effects settings
pub const CENTER_LINE_COLOR: Color = Color::new(0.5, 0.5, 0.5, 0.5); // Color for center line
pub const CENTER_LINE_WIDTH: f32 = 2.0; // Width of center line
pub const CENTER_LINE_DASH_LENGTH: f32 = 10.0; // Length of center line dashes
pub const CENTER_LINE_GAP_LENGTH: f32 = 5.0; // Gap between center line dashes

// Particle effects settings
pub const PARTICLES_ENABLED: bool = true; // Enable particle effects
pub const MAX_PARTICLES: usize = 200; // Maximum number of particles
pub const PADDLE_HIT_PARTICLE_COUNT: usize = 15; // Particles to emit on paddle hit
pub const WALL_HIT_PARTICLE_COUNT: usize = 5; // Particles to emit on wall hit

// Screen shake settings
pub const SCREEN_SHAKE_ENABLED: bool = true; // Enable screen shake effect
pub const SCREEN_SHAKE_INTENSITY: f32 = 5.0; // Maximum screen shake offset
pub const SCREEN_SHAKE_DURATION: f32 = 0.2; // Duration of screen shake in seconds

// Countdown settings
pub const COUNTDOWN_SECONDS: i32 = 3; // Countdown duration before round starts

// Power-up settings
pub const POWERUPS_ENABLED: bool = true; // Enable power-ups
pub const POWERUP_SIZE: f32 = 20.0; // Size of power-up
pub const POWERUP_SPAWN_INTERVAL: f32 = 10.0; // Time between power-up spawns in seconds
pub const POWERUP_DURATION: f32 = 5.0; // Duration of power-up effects in seconds
pub const POWERUP_FLASH_THRESHOLD: f32 = 1.0; // When a power-up starts flashing (seconds remaining)
pub const POWERUP_FLASH_SPEED: f32 = 8.0; // How fast the power-up flashes (cycles per second)

// Game variant settings
pub const BALL_ACCELERATION_FACTOR: f32 = 10.0; // How much the ball accelerates per second
pub const MAX_BALL_SPEED: f32 = 800.0; // Maximum ball speed for acceleration mode
pub const LONG_RALLY_THRESHOLD: i32 = 5; // Number of hits to consider a rally "long"
pub const LONG_RALLY_SPEED_MULTIPLIER: f32 = 1.2; // Speed multiplier for long rallies
