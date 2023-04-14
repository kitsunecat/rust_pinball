use ggez::{
    event::{self, EventHandler, KeyCode, KeyMods},
    graphics::{self, Color},
    mint::Point2,
    Context, ContextBuilder, GameResult,
};

use glam::Vec2;

const WINDOW_WIDTH: f32 = 800.0;
const WINDOW_HEIGHT: f32 = 600.0;

struct PinballGame {
    ball: Ball,
    paddle: Paddle,
}

impl PinballGame {
    pub fn new(ctx: &mut Context) -> GameResult<PinballGame> {
        let ball = Ball::new(ctx)?;
        let paddle = Paddle::new(ctx)?;
        Ok(PinballGame { ball, paddle })
    }
}

impl EventHandler for PinballGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.ball.update(ctx)?;
        self.paddle.update(ctx)?;

        // Add collision detection between the ball and the paddle
        if self.ball.position.y + self.ball.radius >= self.paddle.position.y
            && self.ball.position.x >= self.paddle.position.x
            && self.ball.position.x <= self.paddle.position.x + self.paddle.width
        {
            self.ball.velocity.y = -self.ball.velocity.y;
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, Color::BLACK);
        self.ball.draw(ctx)?;
        self.paddle.draw(ctx)?;
        graphics::present(ctx)?;
        Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        _keymod: KeyMods,
        _repeat: bool,
    ) {
        match keycode {
            KeyCode::Left => self.paddle.velocity.x = -5.0,
            KeyCode::Right => self.paddle.velocity.x = 5.0,
            KeyCode::Escape => ggez::event::quit(ctx),
            _ => (),
        }
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymod: KeyMods) {
        match keycode {
            KeyCode::Left => self.paddle.velocity.x = 0.0,
            KeyCode::Right => self.paddle.velocity.x = 0.0,
            _ => (),
        }
    }
}

struct Ball {
    position: Vec2,
    velocity: Vec2,
    radius: f32,
}

impl Ball {
    pub fn new(_ctx: &mut Context) -> GameResult<Ball> {
        Ok(Ball {
            position: Vec2::new(WINDOW_WIDTH / 2.0, WINDOW_HEIGHT / 2.0),
            velocity: Vec2::new(3.0, 3.0),
            radius: 10.0,
        })
    }

    pub fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.position += self.velocity;
        if self.position.x < 0.0 || self.position.x > WINDOW_WIDTH {
            self.velocity.x = -self.velocity.x;
        }
        if self.position.y < 0.0 || self.position.y > WINDOW_HEIGHT {
            self.velocity.y = -self.velocity.y;
        }
        Ok(())
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult {
        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            Point2::from(self.position.to_array()),
            self.radius,
            0.1,
            graphics::Color::WHITE,
        )?;

        graphics::draw(ctx, &circle, graphics::DrawParam::default())
    }
}

struct Paddle {
    position: Vec2,
    velocity: Vec2,
    width: f32,
    height: f32,
}

impl Paddle {
    fn new(_ctx: &mut Context) -> GameResult<Paddle> {
        let paddle_width = 100.0;
        let paddle_height = 20.0;
        let position = Vec2::new(
            (WINDOW_WIDTH - paddle_width) / 2.0,
            WINDOW_HEIGHT - paddle_height * 2.0,
        );

        Ok(Paddle {
            position,
            velocity: Vec2::new(0.0, 0.0),
            width: paddle_width,   // この行を追加
            height: paddle_height, // この行を追加
        })
    }

    pub fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.position += self.velocity;
        if self.position.x < 0.0 {
            self.position.x = 0.0;
        }
        if self.position.x + self.width > WINDOW_WIDTH {
            self.position.x = WINDOW_WIDTH - self.width;
        }
        Ok(())
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult {
        let rect = graphics::Rect::new(self.position.x, self.position.y, self.width, self.height);
        let paddle =
            graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect, Color::WHITE)?;
        graphics::draw(ctx, &paddle, graphics::DrawParam::default())
    }
}

pub fn main() -> GameResult {
    let (mut ctx, event_loop) = ContextBuilder::new("pinball", "Your Name")
        .window_setup(ggez::conf::WindowSetup::default().title("Pinball Game"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(WINDOW_WIDTH, WINDOW_HEIGHT))
        .build()?;
    let game = PinballGame::new(&mut ctx)?;
    event::run(ctx, event_loop, game)
}
