use ggez::context::{Has, HasMut};
use ggez::glam::*;
use ggez::{Context, ContextBuilder, GameResult};
use ggez::graphics::{self, Color, Rect, DrawParam};
use ggez::event::{self, EventHandler};
use ggez::input::keyboard::{KeyCode, KeyMods, KeyInput};

struct Player {
    p_graphic: graphics::Mesh,
    p_size: Vec2,
    p_pos: Vec2,
    p_vel: Vec2,
    p_max_speed: f32,
    p_grounded: bool,
}

struct Ground {
    g_graphic: graphics::Mesh,
    g_pos: Vec2,
}

struct MyGame {
    player: Player,
    ground: Ground,
    w_width: f32,
    w_heigth: f32,
}

impl MyGame {
    pub fn new(_ctx: &mut Context) -> MyGame {
        // Load/create resources such as images here.
       
        let window_width = 800.;
        let window_heigth = 600.;
        let _p_size = Vec2::new(20., 60.);

  
        let g_graphic = graphics::Mesh::new_rectangle(
            _ctx,
            graphics::DrawMode::fill(), 
            Rect::new(0., 0., window_width, 100.),
            Color::BLACK,
        ).unwrap();

        let ground = Ground { g_graphic, g_pos: Vec2::new(0., window_heigth - 100.) }; 
     
        let rect = graphics::Mesh::new_rectangle(
            _ctx,
            graphics::DrawMode::fill(), 
            Rect::new(0., 0., _p_size.x, _p_size.y),
            Color::RED,
        ).unwrap();

        let player = Player {
            p_graphic: rect,
            p_size: _p_size,
            p_pos: Vec2::new(70., ground.g_pos.y - _p_size.y),
            p_vel: Vec2::new(0., 0.),
            p_max_speed: 270.,
            p_grounded: true,
        };

        MyGame {
            player,
            ground, 
            w_width: window_width,
            w_heigth: window_heigth
        }
    }
}

impl EventHandler for MyGame {


    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        let k_ctx = &_ctx.keyboard;
        let delta_secs = _ctx.time.delta().as_secs_f32();
        let jump_force = 450.;
        let gravity_force = 900.;
        
        let player_rect = Rect::new(
            self.player.p_pos.x,
            self.player.p_pos.y,
            self.player.p_size.x + 1.,  // Used for detecting whether
            self.player.p_size.y + 1., // the player is grounded or not
        );

        let ground_rect = Rect::new(
            0.,
            self.w_heigth - 100.,
            self.w_width,
            100.,
        );

        if k_ctx.is_key_pressed(KeyCode::LShift) {
            self.player.p_max_speed = 500.;
        } else {
            self.player.p_max_speed = 270.;
        }
        
        if k_ctx.is_key_pressed(KeyCode::A) && self.player.p_pos.x > 0. + (self.player.p_size.x / 2.) {
            self.player.p_pos.x -= self.player.p_max_speed * delta_secs;
        }

        if k_ctx.is_key_pressed(KeyCode::D) {
            self.player.p_pos.x += self.player.p_max_speed * delta_secs;
        }
        
        if self.player.p_grounded && k_ctx.is_key_pressed(KeyCode::Space) {
            self.player.p_vel.y = -jump_force;
        }

        // Apply the vel to the player pos
        self.player.p_pos.y += self.player.p_vel.y * delta_secs;
        
        self.player.p_vel.y += gravity_force * delta_secs;
        
        println!("Player velocity is: {}", self.player.p_vel.y);
        if player_rect.overlaps(&ground_rect) {
            self.player.p_grounded = true;
            self.player.p_vel.y = 0.;
        } else {
            self.player.p_grounded = false;
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {

        //ggez::conf::WindowMode::dimensions(self, self.w_width, self.w_heigth);
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::WHITE);
        
        //let ground_pos = self.ground.g_pos.clone();

        canvas.draw(&self.player.p_graphic, self.player.p_pos);
        canvas.draw(&self.ground.g_graphic, self.ground.g_pos);

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
