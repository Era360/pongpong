use ggez::glam::Vec2;
use ggez::graphics::{Color, Drawable, Text, TextFragment};
use ggez::{graphics, Context, GameResult};
use std::time::Duration;

pub struct Countdown {
    pub duration: Duration,
    pub elapsed: Duration,
    pub active: bool,
    pub count: i32,
}

impl Countdown {
    pub fn new(seconds: i32) -> Self {
        Self {
            duration: Duration::from_secs(seconds as u64),
            elapsed: Duration::new(0, 0),
            active: false,
            count: seconds,
        }
    }

    pub fn start(&mut self) {
        self.elapsed = Duration::new(0, 0);
        self.active = true;
        self.count = self.duration.as_secs() as i32;
    }

    pub fn update(&mut self, delta: Duration) -> bool {
        if !self.active {
            return false;
        }

        self.elapsed += delta;

        // Calculate the current countdown number
        let remaining = self.duration.as_secs() as i32 - self.elapsed.as_secs() as i32;
        if remaining != self.count {
            self.count = remaining;
        }

        if self.elapsed >= self.duration {
            self.active = false;
            return true; // Countdown finished
        }

        false // Still counting down
    }

    pub fn draw(&self, ctx: &mut Context, canvas: &mut graphics::Canvas) -> GameResult {
        if !self.active || self.count <= 0 {
            return Ok(());
        }

        let text_value = if self.count > 0 {
            self.count.to_string()
        } else {
            "GO!".to_string()
        };

        let countdown_text = Text::new(
            TextFragment::new(text_value)
                .color(Color::BLACK)
                .scale(graphics::PxScale::from(100.0)),
        );

        let screen_size = ctx.gfx.size();
        let text_dimensions = countdown_text.dimensions(ctx).unwrap_or_default();

        let position = Vec2::new(
            screen_size.0 / 2.0 - text_dimensions.w / 2.0,
            screen_size.1 / 2.0 - text_dimensions.h / 2.0,
        );

        canvas.draw(
            &countdown_text,
            graphics::DrawParam::from(position).color(Color::new(0.0, 0.0, 0.0, 0.8)),
        );

        Ok(())
    }
}
