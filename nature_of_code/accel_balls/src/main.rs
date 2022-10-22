use nannou::prelude::*;
use nannou::color;

struct Model {
    ball: Ball,
}

struct Ball {
    pos: Vec2,
    vel: Vec2,
    acc: Vec2,
    radius: f32,
    color: color::Srgb,
}

impl Ball {

    fn update(&mut self, app: &App) {
        let win = app.window_rect();
        let mouse = app.mouse.position();
        self.update_pos(&win);
        self.update_vel();
        self.update_acc(&mouse);
    }

    fn update_pos(&mut self, win: &Rect) {
        self.pos += self.vel;

        if self.pos.x + self.radius >= win.right() {
            self.pos.x = win.right() - self.radius;
            self.vel.x *= -1.0;
        } else if self.pos.x - self.radius <= win.left() {
            self.pos.x = win.left() + self.radius;
            self.vel.x *= -1.0;
        }

        if self.pos.y + self.radius >= win.top() {
            self.pos.y = win.top() - self.radius;
            self.vel.y *= -1.0;
        } else if self.pos.y - self.radius <= win.bottom() {
            self.pos.y = win.bottom() + self.radius;
            self.vel.y *= -1.0;
        }
    }

    fn update_vel(&mut self) {
        self.vel += self.acc;
        if self.vel.length().abs() > 20.0 {
            self.vel = self.vel.normalize() * 20.0;
        }
    }

    fn update_acc(&mut self, mouse: &Vec2) {
        let acc = (*mouse - self.pos) * 0.01;
        if !acc.x.is_nan() && !acc.y.is_nan() {
            self.acc = acc;
        }
    }

    fn display(&self, draw: &mut Draw) {
        draw.ellipse()
            .xy(self.pos)
            .wh(Vec2::new(2.0 * self.radius, 2.0 * self.radius))
            .color(WHITESMOKE);
    }
}

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

fn model(_app: &App) -> Model {
    Model {
        ball: Ball{
            pos: Vec2::new(0.0, 0.0),
            vel: Vec2::new(1.0, 1.0).normalize() * 10.0,
            acc: Vec2::new(0.0, 0.0),
            radius: 50.0,
        }
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    model.ball.update(app);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(SLATEGREY);

    let ball = &model.ball;

    draw.to_frame(&app, &frame).unwrap();
}
