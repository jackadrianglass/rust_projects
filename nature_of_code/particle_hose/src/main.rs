use nannou::color::rgba;
use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};

#[derive(Clone, Copy)]
struct Particle {
    pos: Vec2,
    vel: Vec2,
    acc: Vec2,

    rotation: f32,
    angular_vel: f32,

    lifetime: f32,
}

impl Particle {
    fn new() -> Self {
        Self {
            pos: Vec2::new(0.0, 200.0),
            vel: Vec2::new(random_f32() * 8.0 - 4.0, random_f32() * 5.0),
            acc: Vec2::new(0.0, -0.15),
            rotation: 0.0,
            angular_vel: random_f32() * 10.0,
            lifetime: 250.0,
        }
    }

    fn update(&mut self) {
        self.pos += self.vel;
        self.vel += self.acc;
        self.rotation += self.angular_vel / 100.0;

        if !self.is_dead() {
            self.lifetime -= 2.5;
        }
    }

    fn is_dead(&self) -> bool {
        self.lifetime <= 0.0
    }

    fn draw(&self, draw: &Draw) {
        draw.rect()
            .wh(Vec2::new(50.0, 50.0))
            .xy(self.pos)
            .rotate(self.rotation)
            .stroke(rgba(0.0, 0.0, 0.0, self.lifetime / 250.0))
            .stroke_weight(5.0)
            .color(rgba(0.5, 0.5, 0.5, self.lifetime / 250.0));
    }
}

struct Model {
    particles: Vec<Particle>,
    egui: Egui,
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

    let particles = Vec::with_capacity(200);

    Model { egui, particles }
}

fn update(_app: &App, model: &mut Model, update: Update) {
    let egui = &mut model.egui;

    egui.set_elapsed_time(update.since_start);
    let ctx = egui.begin_frame();

    egui::Window::new("Settings").show(&ctx, |ui| {
        ui.label("Settings");

        // TODO: egui gui elements go here
    });

    // TODO: update logic goes here
    for p in model.particles.iter_mut() {
        p.update();
    }

    model.particles.retain(|p| !p.is_dead());
    if model.particles.len() < 200 {
        model.particles.push(Particle::new());
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(SLATEGREY);

    // TODO: drawing logic goes here
    for p in model.particles.iter() {
        p.draw(&draw);
    }

    draw.to_frame(&app, &frame).unwrap();
    model.egui.draw_to_frame(&frame).unwrap();
}
