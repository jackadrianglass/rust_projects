use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};

#[derive(Debug)]
struct Star {
    pos: Vec3,
    prev_z: f32,
}

impl Star {
    fn new_point() -> Vec3 {
        let d = || (random_f32() - 0.5) * 2.0;
        Vec3::new(d(), d(), 1.0)
    }
    fn new() -> Self {
        Self {
            pos: Self::new_point(),
            prev_z: 1.0,
        }
    }

    fn xr(&self) -> f32 {
        self.pos.x / self.pos.z
    }

    fn yr(&self) -> f32 {
        self.pos.y / self.pos.z
    }

    fn pxr(&self) -> f32 {
        self.pos.x / self.prev_z
    }

    fn pyr(&self) -> f32 {
        self.pos.y / self.prev_z
    }

    fn update(&mut self, speed: f32) {
        self.prev_z = self.pos.z;
        self.pos.z -= speed;
        if self.pos.z <= 0.0 || self.xr().abs() > 1.0 || self.yr().abs() > 1.0 {
            self.pos = Self::new_point();
            self.prev_z = self.pos.z;
        }
    }

    fn show(&self, draw: &Draw, win: &Rect) {
        let xy = Vec2::new(self.xr() * win.right(), self.yr() * win.top());
        let pxy = Vec2::new(self.pxr() * win.right(), self.pyr() * win.top());
        let weight = (1.0 - self.pos.z) * 8.0;
        let wh = Vec2::new(1.0, 1.0) * weight;

        draw.ellipse().xy(xy).wh(wh).color(WHITE);
        draw.line().start(pxy).end(xy).weight(weight).color(WHITE);
    }
}
//========================================================================

struct Model {
    egui: Egui,
    stars: Vec<Star>,
    speed: f32,
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

    let mut stars = Vec::new();
    for _ in 0..100 {
        stars.push(Star::new());
    }

    Model {
        egui,
        stars,
        speed: 0.01,
    }
}

fn update(_app: &App, model: &mut Model, update: Update) {
    let egui = &mut model.egui;

    egui.set_elapsed_time(update.since_start);
    let ctx = egui.begin_frame();

    for star in model.stars.iter_mut() {
        star.update(model.speed);
    }

    egui::Window::new("Settings").show(&ctx, |ui| {
        ui.add(egui::Slider::new(&mut model.speed, 0.0001..=0.1).text("Speeeeeeed"));
    });
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    let win = app.main_window().rect();
    for star in model.stars.iter() {
        star.show(&draw, &win);
    }

    draw.to_frame(&app, &frame).unwrap();
    model.egui.draw_to_frame(&frame).unwrap();
}
