use nannou::prelude::*;
use nannou::color::rgb;
use nannou_egui::{self, egui, Egui};

/*
Bouncing balls
- Want perfect collisions for a start
- want a single bouncing ball to start
    - bounces off the ground
    - bounces off the wall
- Two balls bounce off each other
- N balls be bouncing
*/

#[derive(Clone)]
struct Ball {
    mass: f32,
    radius: f32,
    pos: Vec2,
    vel: Vec2,
}

impl Ball {
    fn random(win: &Rect) -> Self {
        let radius = random_range(10.0, 50.0);
        Self {
            mass: random_range(25.0, 75.0),
            radius,
            pos: vec2(
                random_range(win.left() + radius, win.right() - radius),
                random_range(win.bottom() + radius, win.top() - radius)
            ),
            vel: vec2(random_range(-100.0, 100.0) * 2.0, random_range(-100.0, 100.0) * 2.0)
        }
    }
    fn intersects(&self, other: &Self) -> bool {
        (other.pos - self.pos).length() < other.radius + self.radius
    }

    fn update(&mut self, app: &App) {
        let delta = app.duration.since_prev_update.as_secs_f32();

        self.pos += self.vel * delta;

        // window boundaries
        let win = app.window_rect();
        if self.pos.x + self.radius > win.right() {
            self.pos.x = win.right() - self.radius;
            self.vel.x *= -1.0;
        }

        if self.pos.x - self.radius < win.left() {
            self.pos.x = win.left() + self.radius;
            self.vel.x *= -1.0;
        }

        if self.pos.y + self.radius > win.top() {
            self.pos.y = win.top() - self.radius;
            self.vel.y *= -1.0;
        }

        if self.pos.y - self.radius < win.bottom() {
            self.pos.y = win.bottom() + self.radius;
            self.vel.y *= -1.0;
        }
    }

    fn draw(&self, draw: &Draw) {
        let wh = vec2(2.0 * self.radius, 2.0 * self.radius);
        let c = if self.mass > 255.0 { 1.0 } else { self.mass / 255.0 };
        let color = rgb(c, c, c);
        draw.ellipse().xy(self.pos).wh(wh).color(color);
    }
}

fn collision_update(a: &mut Ball, b: &mut Ball) {
    let total_mass = a.mass + b.mass;
    let new_vel = |p1: &Ball, p2: &Ball| {
        let d = (p1.pos - p2.pos).length().powf(2.0);
        let m = 2.0 * p2.mass / total_mass;
        let v = (p1.vel - p2.vel).dot(p1.pos - p2.pos);
        p1.vel - m * (v / d) * (p1.pos - p2.pos)
    };
    let new_a = new_vel(&a, &b);
    let new_b = new_vel(&b, &a);
    a.vel = new_a;
    b.vel = new_b;
}

struct Model {
    egui: Egui,
    balls: Vec<Ball>,
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
    let win = window.rect();

    let egui = Egui::from_window(&window);
    let mut balls = vec![Ball::random(&win)];
    while balls.len() < 25 {
        let ball = Ball::random(&win);
        if !balls.iter().any(|b| b.intersects(&ball)) {
            balls.push(ball);
        }
    }

    Model {
        egui,
        balls
    }
}

fn update(app: &App, model: &mut Model, update: Update) {
    let egui = &mut model.egui;

    egui.set_elapsed_time(update.since_start);
    let ctx = egui.begin_frame();

    egui::Window::new("Settings").show(&ctx, |ui| {
        ui.label("Settings");
        // TODO: egui gui elements go here
    });

    for ball in model.balls.iter_mut() {
        ball.update(&app);
    }

    // collisions
    for i in 0..model.balls.len() {
        for j in i+1..model.balls.len() {
            let mut a = model.balls.get(i).unwrap().clone();
            let mut b = model.balls.get_mut(j).unwrap().clone();
            if a.intersects(&b) {
                collision_update(&mut a, &mut b);
            }
            let _ = std::mem::replace(&mut model.balls[i], a);
            let _ = std::mem::replace(&mut model.balls[j], b);
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(SLATEGREY);

    // TODO: drawing logic goes here
    for ball in model.balls.iter() {
        ball.draw(&draw);
    }

    draw.to_frame(&app, &frame).unwrap();
    model.egui.draw_to_frame(&frame).unwrap();
}
