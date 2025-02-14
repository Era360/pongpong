use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Color, DrawParam, Text};
use ggez::{Context, ContextBuilder, GameResult};

fn main() {
    // Make a Context.
    let (mut ctx, event_loop) = ContextBuilder::new("my_game", "Cool Game Author")
        .build()
        .expect("aieee, could not create ggez context!");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let my_game = MyGame::new(&mut ctx);

    // Run!
    event::run(ctx, event_loop, my_game);
}

struct MyGame {
    // Your state here...
}

impl MyGame {
    pub fn new(_ctx: &mut Context) -> MyGame {
        // Load/create resources such as images here.
        MyGame {
            // ...
        }
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        // Update code here...
        println!("update");
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);
        // Draw code here...

        // Get the current FPS
        let fps = ctx.time.fps();

        // Create a text object to display the FPS
        let fps_display = Text::new(format!("FPS: {:.2}", fps));

        // Draw the FPS text on the screen
        canvas.draw(&fps_display, DrawParam::default().dest([10.0, 10.0]));

        // Finish drawing
        canvas.finish(ctx)
    }
}
