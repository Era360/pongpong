use super::powerup::{PowerUp, PowerUpType};
use crate::constants::POWERUP_SPAWN_INTERVAL;
use crate::entities::ball::Ball;
use crate::entities::player::Player;
// use ggez::glam::Vec2;
use ggez::graphics;
use ggez::{Context, GameResult};
use std::time::Duration;

pub struct PowerUpManager {
    power_ups: Vec<PowerUp>,
    spawn_timer: Duration,
    total_time: f32, // Used for visual effects
}

impl PowerUpManager {
    pub fn new() -> Self {
        Self {
            power_ups: Vec::new(),
            spawn_timer: Duration::from_secs_f32(POWERUP_SPAWN_INTERVAL),
            total_time: 0.0,
        }
    }

    pub fn update(
        &mut self,
        delta: Duration,
        ball: &Ball,
        left_player: &mut Player,
        right_player: &mut Player,
    ) {
        self.total_time += delta.as_secs_f32();

        // Update spawn timer
        self.spawn_timer = self.spawn_timer.saturating_sub(delta);
        if self.spawn_timer.as_secs_f32() <= 0.0 {
            // Spawn a new power-up
            self.spawn_power_up();
            self.spawn_timer = Duration::from_secs_f32(POWERUP_SPAWN_INTERVAL);
        }

        // Use a separate vector to track which power-ups were collected
        let mut collected_power_ups = Vec::new();

        // Check for ball collisions with active power-ups
        for (i, power_up) in self.power_ups.iter_mut().enumerate() {
            if power_up.active && power_up.collides_with_ball(ball.position, ball.radius) {
                // Determine which player gets the power-up based on ball direction
                let player_index = if ball.velocity.x > 0.0 { 1 } else { 0 };
                power_up.activate(player_index);

                // Store which power-up was collected for later processing
                collected_power_ups.push((i, power_up.power_type, player_index));
            }
        }

        // Apply power-up effects after the loop
        for (_, power_type, player_index) in collected_power_ups {
            match power_type {
                PowerUpType::PaddleGrow => {
                    if player_index == 0 {
                        left_player.apply_size_multiplier(1.5);
                    } else {
                        right_player.apply_size_multiplier(1.5);
                    }
                }
                PowerUpType::PaddleShrink => {
                    if player_index == 0 {
                        right_player.apply_size_multiplier(0.75);
                    } else {
                        left_player.apply_size_multiplier(0.75);
                    }
                }
                PowerUpType::SpeedUp => {
                    if player_index == 0 {
                        left_player.apply_speed_multiplier(1.5);
                    } else {
                        right_player.apply_speed_multiplier(1.5);
                    }
                }
                PowerUpType::SpeedDown => {
                    if player_index == 0 {
                        right_player.apply_speed_multiplier(0.75);
                    } else {
                        left_player.apply_speed_multiplier(0.75);
                    }
                }
                PowerUpType::MultiballSplit => {
                    // This is handled in the main game logic
                }
            }
        }

        // Update active power-ups and remove expired ones
        self.power_ups.retain_mut(|power_up| power_up.update(delta));
    }

    fn spawn_power_up(&mut self) {
        self.power_ups.push(PowerUp::random());
    }

    pub fn draw(&self, ctx: &mut Context, canvas: &mut graphics::Canvas) -> GameResult {
        for power_up in &self.power_ups {
            power_up.draw(ctx, canvas, self.total_time)?;
        }
        Ok(())
    }

    pub fn get_active_multiball(&self) -> bool {
        self.power_ups.iter().any(|p| {
            p.power_type == PowerUpType::MultiballSplit
                && p.collected_by.is_some()
                && p.remaining_duration.is_some()
        })
    }

    pub fn reset(&mut self) {
        self.power_ups.clear();
        self.spawn_timer = Duration::from_secs_f32(POWERUP_SPAWN_INTERVAL);
    }
}
