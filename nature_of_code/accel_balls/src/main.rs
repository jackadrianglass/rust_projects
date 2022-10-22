use nannou::prelude::*;
use nannou::color;
use nannou_egui::{self, egui, Egui};

struct Model {
    egui: Egui,
    balls: Vec<Ball>,
}

struct Ball {
    pos: Vec2,
    vel: Vec2,
    top_speed: f32,
    acc: Vec2,
    acc_mag: f32,
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
        if self.vel.length().abs() > self.top_speed {
            self.vel = self.vel.normalize() * self.top_speed;
        }
    }

    fn update_acc(&mut self, mouse: &Vec2) {
        let acc = (*mouse - self.pos).normalize() * self.acc_mag;
        if !acc.x.is_nan() && !acc.y.is_nan() {
            self.acc = acc;
        }
    }

    fn display(&self, draw: &Draw) {
        draw.ellipse()
            .xy(self.pos)
            .wh(Vec2::new(2.0 * self.radius, 2.0 * self.radius))
            .color(self.color);
    }
}

fn main() {
    nannou::app(model).update(update).run();
}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    // Let egui handle things like keyboard and mouse input.
    model.egui.handle_raw_event(event);
}

fn model(app: &App) -> Model {
    let window_id = app
        .new_window()
        .view(view)
        .raw_event(raw_window_event)
        .build()
        .unwrap();
    let window = app.window(window_id).unwrap();

    let egui = Egui::from_window(&window);

    Model {
        egui,
        balls: vec![
            Ball{
                pos: Vec2::new(0.0, 0.0),
                vel: Vec2::new(1.0, 1.0).normalize() * 10.0,
                top_speed: 20.0,
                acc: Vec2::new(0.0, 0.0),
                acc_mag: 0.5,
                radius: 50.0,
                color: rgb(1.0, 1.0, 1.0),
            },
            Ball{
                pos: Vec2::new(0.0, 0.0),
                vel: Vec2::new(2.0, -1.0).normalize() * 10.0,
                top_speed: 20.0,
                acc: Vec2::new(0.0, 0.0),
                acc_mag: 1.0,
                radius: 30.0,
                color: rgb(0.0, 1.0, 1.0),
            },
        ]
    }
}

fn update(app: &App, model: &mut Model, update: Update) {
    let egui = &mut model.egui;

    egui.set_elapsed_time(update.since_start);
    let ctx = egui.begin_frame();

    egui::Window::new("Settings").show(&ctx, |ui| {
        for ball in model.balls.iter_mut() {
            ui.label("Ball Accel");
            ui.add(egui::Slider::new(&mut ball.acc_mag, 0.01..=10.0));
            ui.label("Ball top speed");
            ui.add(egui::Slider::new(&mut ball.top_speed, 1.0..=100.0));
        }
    });
    for ball in model.balls.iter_mut() {
        ball.update(app);
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(SLATEGREY);

    for ball in model.balls.iter() {
        ball.display(&draw);
    }

    draw.to_frame(&app, &frame).unwrap();
    model.egui.draw_to_frame(&frame).unwrap();
}
