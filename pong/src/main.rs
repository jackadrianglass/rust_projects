use ggez::conf::{NumSamples, WindowMode, WindowSetup};
use ggez::event;
use ggez::graphics::{self, Color, Rect, Text};
use ggez::{Context, GameResult};
use glam::*;

struct MainState {
    width: f32,
    height: f32,
    score: Vec2,
    score_text: Text,

    paddle_bounds: Rect,
    left: Vec2,
    right: Vec2,
    ldir: f32,
    rdir: f32,

    ball_bounds: Rect,
    ball: Vec2,
    bdir: Vec2,
}

impl MainState {
    fn new(width: f32, height: f32) -> GameResult<MainState> {
        let w = width / 40.0;
        let h = height / 8.0;
        let s = MainState {
            width,
            height,
            score: Vec2::default(),
            score_text: Text::new("0 - 0"),
            paddle_bounds: Rect::new(0.0, 0.0, w, h),
            left: Vec2::new(0.0, 0.0),
            right: Vec2::new(width - w, height / 2.0),
            ldir: 0.0,
            rdir: 0.0,
            ball_bounds: Rect::new(0.0, 0.0, w, w),
            ball: Vec2::new(width / 2.0, height / 2.0),
            bdir: Vec2::new(1.0, 1.0),
        };
        Ok(s)
    }
}

impl MainState {
    fn update_paddles(&mut self) {
        const SPEED: f32 = 5.0;

        self.left.y += self.ldir * SPEED;
        if self.left.y < 0.0 {
            self.left.y = 0.5;
        } else if self.left.y > self.height - self.paddle_bounds.h {
            self.left.y = self.height - self.paddle_bounds.h - 0.5;
        }

        self.right.y += self.rdir * SPEED;
        if self.right.y < 0.0 {
            self.right.y = 0.5;
        } else if self.right.y > self.height - self.paddle_bounds.h {
            self.right.y = self.height - self.paddle_bounds.h - 0.5;
        }
    }

    fn update_ball(&mut self) {
        const SPEED: f32 = 2.0;

        self.ball += self.bdir * SPEED;

        // top and bottom walls
        if self.ball.y < 0.0 {
            self.bdir.y = 1.0;
            self.ball.y = 0.0;
        } else if self.ball.y > self.height - self.ball_bounds.h {
            self.bdir.y = -1.0;
            self.ball.y = self.height - self.ball_bounds.h;
        }

        if self.intersects(&self.ball, &self.left) {
            self.bdir.x = 1.0;
            self.ball.x = self.paddle_bounds.w + 0.5;
        } else if self.intersects(&self.ball, &self.right) {
            self.bdir.x = -1.0;
            self.ball.x = self.width - self.paddle_bounds.w - self.ball_bounds.w - 0.5;
        }

        // point conditions
        if self.ball.x < 0.0 {
            self.ball = Vec2::new(self.width / 2.0, self.height / 2.0);
            self.score.x += 1.0;
            self.score_text = Text::new(format!("{} - {}", self.score.x, self.score.y).to_owned());
        } else if self.ball.x > self.width {
            self.ball = Vec2::new(self.width / 2.0, self.height / 2.0);
            self.score.y += 1.0;
            self.score_text = Text::new(format!("{} - {}", self.score.x, self.score.y).to_owned());
        }
    }

    fn intersects(&self, ball: &Vec2, paddle: &Vec2) -> bool {
        ball.x + self.ball_bounds.w >= paddle.x
            && ball.x <= paddle.x + self.paddle_bounds.w
            && ball.y + self.ball_bounds.h >= paddle.y
            && ball.y <= paddle.y + self.paddle_bounds.h
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: event::KeyCode,
        _keymods: event::KeyMods,
        repeat: bool,
    ) {
        if repeat {
            return;
        }

        match keycode {
            event::KeyCode::Up => self.rdir -= 1.0,
            event::KeyCode::Down => self.rdir += 1.0,
            event::KeyCode::Left => self.ldir -= 1.0,
            event::KeyCode::Right => self.ldir += 1.0,
            _ => {}
        }
    }

    fn key_up_event(
        &mut self,
        _ctx: &mut Context,
        keycode: event::KeyCode,
        _keymods: event::KeyMods,
    ) {
        match keycode {
            event::KeyCode::Up => self.rdir += 1.0,
            event::KeyCode::Down => self.rdir -= 1.0,
            event::KeyCode::Left => self.ldir += 1.0,
            event::KeyCode::Right => self.ldir -= 1.0,
            _ => {}
        }
    }

    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.update_paddles();
        self.update_ball();

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        let paddle = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            self.paddle_bounds,
            Color::WHITE,
        )?;

        let ball = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            self.ball_bounds,
            Color::GREEN,
        )?;

        graphics::draw(
            ctx,
            &self.score_text,
            (Vec2::new(
                (self.width / 2.0) - (self.score_text.width(&ctx) / 2.0),
                0.0,
            ),),
        )?;
        graphics::draw(ctx, &paddle, (self.left,))?;
        graphics::draw(ctx, &paddle, (self.right,))?;
        graphics::draw(ctx, &ball, (self.ball,))?;
        graphics::present(ctx)?;
        Ok(())
    }
}

fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("super_simple", "ggez");
    let cb = cb.window_setup(WindowSetup {
        title: "Pong".to_owned(),
        samples: NumSamples::One,
        vsync: true,
        icon: "".to_owned(),
        srgb: true,
    });
    let m = WindowMode::default();
    let cb = cb.window_mode(m);
    let (ctx, event_loop) = cb.build()?;
    let state = MainState::new(m.width, m.height)?;
    event::run(ctx, event_loop, state);
}
