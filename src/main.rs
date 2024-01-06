use ggez::glam::*;
use ggez::{Context, ContextBuilder, GameResult};
use ggez::graphics::{self, Color, Rect, DrawParam};
use ggez::event::{self, EventHandler};

struct MyGame {
    player: graphics::Mesh,
}

impl MyGame {
    pub fn new(_ctx: &mut Context) -> MyGame {
        // Load/create resources such as images here.
        
        let rect = graphics::Mesh::new_rectangle(
            _ctx,
            graphics::DrawMode::fill(), 
            Rect::new(0., 0., 10., 10.),
            Color::RED,
        ).unwrap();
     
    
        MyGame {player: rect}
    }
}

impl EventHandler for MyGame {

    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::WHITE);
        
        canvas.draw(&self.player, Vec2::new(200., 200.));

        canvas.finish(ctx)
    }
}


fn main() {
    // Make a Context.
    let (mut ctx, event_loop) = ContextBuilder::new("2D_Platformer", "Aki")
        .build()
        .expect("aieee, could not create ggez context!");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let game = MyGame::new(&mut ctx);

    // Run!
    event::run(ctx, event_loop, game);
}
