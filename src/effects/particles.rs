use ggez::glam::Vec2;
use ggez::graphics::Color;
use ggez::{graphics, Context, GameResult};
use rand::{rng, Rng};
use std::time::Duration;

pub struct Particle {
    pub position: Vec2,
    pub velocity: Vec2,
    pub color: Color,
    pub size: f32,
    pub lifetime: Duration,
    pub elapsed: Duration,
}

impl Particle {
    pub fn new(position: Vec2, color: Color) -> Self {
        let mut rng = rng();

        // Random velocity in circular pattern
        let angle = rng.random_range(0.0..std::f32::consts::TAU);
        let speed = rng.random_range(50.0..200.0);

        Self {
            position,
            velocity: Vec2::new(angle.cos() * speed, angle.sin() * speed),
            color,
            size: rng.random_range(1.0..3.0),
            lifetime: Duration::from_millis(rng.random_range(200..500)),
            elapsed: Duration::new(0, 0),
        }
    }

    pub fn update(&mut self, delta: Duration) -> bool {
        self.elapsed += delta;
        if self.elapsed >= self.lifetime {
            return false; // Particle expired
        }

        // Update position
        let delta_seconds = delta.as_secs_f32();
        self.position += self.velocity * delta_seconds;

        // Calculate fade based on lifetime
        let life_ratio = self.elapsed.as_secs_f32() / self.lifetime.as_secs_f32();
        self.color.a = 1.0 - life_ratio;

        // Add some gravity effect
        self.velocity.y += 200.0 * delta_seconds;

        true // Particle still alive
    }
}

pub struct ParticleSystem {
    particles: Vec<Particle>,
    max_particles: usize,
}

impl ParticleSystem {
    pub fn new(max_particles: usize) -> Self {
        Self {
            particles: Vec::with_capacity(max_particles),
            max_particles,
        }
    }

    pub fn emit(&mut self, position: Vec2, color: Color, count: usize) {
        for _ in 0..count {
            if self.particles.len() >= self.max_particles {
                self.particles.remove(0); // Remove oldest particle
            }

            self.particles.push(Particle::new(position, color));
        }
    }

    pub fn update(&mut self, delta: Duration) {
        self.particles.retain_mut(|particle| particle.update(delta));
    }

    pub fn draw(&self, ctx: &mut Context, canvas: &mut graphics::Canvas) -> GameResult {
        for particle in &self.particles {
            if particle.size <= 0.0 {
                continue;
            }

            let circle = graphics::Mesh::new_circle(
                ctx,
                graphics::DrawMode::fill(),
                particle.position,
                particle.size,
                0.1,
                particle.color,
            )?;

            canvas.draw(&circle, graphics::DrawParam::default());
        }

        Ok(())
    }
}
