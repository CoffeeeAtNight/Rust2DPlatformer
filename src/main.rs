use ggez::glam::*;
use ggez::{Context, ContextBuilder, GameResult};
use ggez::graphics::{self, Color, Rect, Image, DrawParam, Mesh, DrawMode, Canvas, Text, PxScale};
use ggez::event::{self, EventHandler};
use ggez::input::keyboard::KeyCode; 

struct Player {
    p_graphic: Mesh,
    p_size: Vec2,
    p_pos: Vec2,
    p_vel: Vec2,
    p_max_speed: f32,
    p_grounded: bool,
    p_spawn_pos: Vec2,
}

struct Ground {
    g_graphic: Mesh,
    g_pos: Vec2,
}

impl Ground {
    fn new(g_graphic: Mesh, x_pos: f32, y_pos: f32) -> Ground {
        Ground { g_graphic, g_pos: Vec2::new(x_pos, y_pos) }
    }
}

struct MyGame {
    player: Player,
    ground: Ground,
    ground_2: Ground,
    ground_3: Ground,
    goal: Ground,
    w_width: f32,
    w_height: f32,
    img_bg: Image,
    goal_reached: bool
}

impl MyGame {
    pub fn new(_ctx: &mut Context) -> MyGame {
        let w_width = 1920.;
        let w_height = 1080.;
        let goal_reached = false;
        let _p_size = Vec2::new(20., 60.);
        let img_bg = Image::from_path(_ctx, "\\background.jpg").unwrap(); 
        let g_graphic = Mesh::new_rectangle(
            _ctx,
            DrawMode::fill(), 
            Rect::new(0., 0., 150., 35.),
            Color::WHITE,
        ).unwrap();
        let g_graphic2 = g_graphic.clone();
        let g_graphic3 = g_graphic.clone();
        let g_graphic4 = g_graphic.clone();
       
        let ground = Ground::new(g_graphic, 0., w_height/2.);
        let ground_2 = Ground::new(g_graphic2, 730., w_height/2. + 130.);
        let ground_3 = Ground::new(g_graphic3, 1260., w_height/2. - 50.);
        let goal = Ground::new(g_graphic4, 1650., w_height - 200.); 
        
        let p_graphic = Mesh::new_rectangle(
            _ctx,
            graphics::DrawMode::fill(), 
            Rect::new(0., 0., _p_size.x, _p_size.y),
            Color::RED,
        ).unwrap();

        let player_default_position = Vec2::new(70., ground.g_pos.y - _p_size.y);
        let player = Player {
            p_graphic,
            p_size: _p_size,
            p_pos: player_default_position,
            p_vel: Vec2::new(0., 0.),
            p_max_speed: 270.,
            p_grounded: true,
            p_spawn_pos: player_default_position,
        };
        
        MyGame {
            player,
            ground,
            ground_2,
            ground_3,
            goal,
            w_width,
            w_height,
            img_bg,
            goal_reached,
        }
    }

    fn player_controls_handler(&mut self, _ctx: &mut Context, delta_secs: f32) {
        let k_ctx = &_ctx.keyboard;
        let jump_force = 450.;

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

        if k_ctx.is_key_pressed(KeyCode::R) {
            self.player.p_pos = self.player.p_spawn_pos;
        }

        if self.player.p_grounded && k_ctx.is_key_pressed(KeyCode::Space) {
            self.player.p_vel.y = -jump_force;
        }

    }

}

impl EventHandler for MyGame {

    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        let delta_secs = _ctx.time.delta().as_secs_f32();
        let gravity_force = 900.;

        let player_p_graphic = Rect::new(
            self.player.p_pos.x,
            self.player.p_pos.y,
            self.player.p_size.x + 1.,  // Used for detecting whether
            self.player.p_size.y + 1., // the player is grounded or not
        );

        let ground_p_graphic = Rect::new(
            self.ground.g_pos.x,
            self.ground.g_pos.y, 
            150.,
            35.,
        );

        let ground_p_graphic_2 = Rect::new(
            self.ground_2.g_pos.x,
            self.ground_2.g_pos.y,
            150.,
            35.,
        );

        let ground_p_graphic_3 = Rect::new(
            self.ground_3.g_pos.x,
            self.ground_3.g_pos.y,
            150.,
            35.,
        );

        let goal_graphic = Rect::new(
            self.goal.g_pos.x,
            self.goal.g_pos.y,
            150.,
            35.,
        );

        self.player_controls_handler(_ctx, delta_secs);

        // Apply the vel to the player pos
        self.player.p_pos.y += self.player.p_vel.y * delta_secs;
        
        self.player.p_vel.y += gravity_force * delta_secs;

        // Player fell of the map
        if self.player.p_pos.y > 1090. {
            self.player.p_pos = self.player.p_spawn_pos;
            // Implement AI punish
        }
        
        // Player ground checks
        if player_p_graphic.overlaps(&ground_p_graphic) 
        || player_p_graphic.overlaps(&ground_p_graphic_2)
        || player_p_graphic.overlaps(&ground_p_graphic_3) {
            self.player.p_grounded = true;
            self.player.p_vel.y = 0.;
        } else {
            self.player.p_grounded = false;
        }

        // Goal conditions
        self.goal_reached = player_p_graphic.overlaps(&goal_graphic);
        if self.goal_reached {
            self.player.p_grounded = true;
            self.player.p_vel.y = 0.;
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = Canvas::from_frame(ctx, Color::WHITE);
        
        canvas.draw(&self.img_bg, DrawParam::new());
        canvas.draw(&self.player.p_graphic, self.player.p_pos);
        canvas.draw(&self.ground.g_graphic, self.ground.g_pos);
        canvas.draw(&self.ground_2.g_graphic, self.ground_2.g_pos);
        canvas.draw(&self.ground_3.g_graphic, self.ground_3.g_pos);
        canvas.draw(&self.goal.g_graphic, self.goal.g_pos);
        canvas.draw(&Text::new("You don't really jump onto me..."), Vec2::new(1200., self.w_height/2. - 150.));

        if self.goal_reached {
            let mut text = Text::new("You won!\nHit 'r' to restart the game.");
            text.set_scale(PxScale::from(24.));
            canvas.draw(&text, Vec2::new(self.w_width/2. - 200., self.w_height/2. - 300.))
        }

        canvas.finish(ctx)
    }
}


fn main() {
    let (mut ctx, event_loop) = ContextBuilder::new("2D_Platformer", "Aki")
        .build()
        .expect("aieee, could not create ggez context!");

   let game = MyGame::new(&mut ctx);

    // Run!
    event::run(ctx, event_loop, game);
}
