use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};

struct Forcefield {
    pos: Vec2,
    radius: f32,
    force: f32,
}

impl Forcefield {
    fn update(&mut self, app: &App) {
        self.pos = app.mouse.position();
    }
}

struct Ball {
    radius: f32,
    pos: Vec2,

    top_speed: f32,
    vel: Vec2,

    drag: f32,
}

impl Ball {
    fn update(&mut self, win: Rect) {
        let drag_mag = if self.vel.length() < self.drag { (self.vel.length() * 10.0).round() / 10.0 } else { self.drag };
        let correction = 15.0;

        self.vel -= self.vel.normalize() * drag_mag / 2.0;

        self.pos += self.vel / correction;

        if self.pos.x > win.right() {
            self.pos.x = win.left();
        } else if self.pos.x < win.left() {
            self.pos.x = win.right();
        } else if self.pos.y > win.top() {
            self.pos.y = win.bottom();
        } else if self.pos.y < win.bottom() {
            self.pos.y = win.top();
        }
    }

    fn draw(&self, app: &App, draw: &Draw) {
        let wh = vec2(self.radius * 2.0, self.radius * 2.0);
        draw.ellipse().xy(self.pos).wh(wh).color(TURQUOISE);

        let win = app.window_rect();
        let pos = &self.pos;
        let r = self.radius;

        let has_x_overlap = pos.x + r > win.right() || pos.x - r < win.left();
        if has_x_overlap {
            let positive = if pos.x.is_sign_negative() { 1.0 } else { -1.0 };
            let new_pos = vec2(positive * win.w() + pos.x, pos.y);
            draw.ellipse().xy(new_pos).wh(wh).color(TURQUOISE);
        }

        let has_y_overlap = pos.y + r > win.top() || pos.y - r < win.bottom();
        if has_y_overlap {
            let positive = if pos.y.is_sign_negative() { 1.0 } else { -1.0 };
            let new_pos = vec2(pos.x, positive * win.h() + pos.y);
            draw.ellipse().xy(new_pos).wh(wh).color(TURQUOISE);
        }

        if has_y_overlap && has_x_overlap {
            let x_pos = if pos.x.is_sign_negative() { 1.0 } else { -1.0 };
            let y_pos = if pos.y.is_sign_negative() { 1.0 } else { -1.0 };

            let new_pos = vec2(x_pos * win.w() + pos.x, y_pos * win.h() + pos.y);
            draw.ellipse().xy(new_pos).wh(wh).color(TURQUOISE);
        }
    }

    fn add_force(&mut self, force: Vec2) {
        self.vel += force / 5.0; // ignoring mass for now
        if self.vel.length() > self.top_speed {
            self.vel = self.vel.normalize() * self.top_speed;
        }
    }
}

fn interesects(ball: &Ball, field: &Forcefield) -> bool {
    field.pos.distance(ball.pos).abs() < field.radius + ball.radius
}

struct Model {
    ball: Ball,
    mouse: Forcefield,
    egui: Egui,
}

fn main() {
    nannou::app(model).update(update).run();
}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
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
        ball: Ball{
            radius: 100.0,
            pos: vec2(0.0, 0.0),
            top_speed: 50.0,
            vel: vec2(5.0, 5.0),
            drag: 1.0,
        },
        mouse: Forcefield { pos: vec2(0.0, 0.0), radius: 50.0, force: 100.0 },
    }
}

fn update(app: &App, model: &mut Model, update: Update) {
    let egui = &mut model.egui;

    egui.set_elapsed_time(update.since_start);
    let ctx = egui.begin_frame();

    egui::Window::new("Settings").show(&ctx, |ui| {
        ui.label("Settings");
        ui.label(format!("Ball vel {:?}", model.ball.vel));
        ui.label(format!("Ball vel length {}", model.ball.vel.length()));

        ui.add(egui::Slider::new(&mut model.ball.radius, 10.0..=400.0).text("Ball Radius"));
        ui.add(egui::Slider::new(&mut model.ball.drag, 0.05..=20.0).text("Ball Drag"));

        ui.add(egui::Slider::new(&mut model.mouse.force, 0.05..=3000.0).text("Forcefield Force"));
   });

    // TODO: update logic goes here
    model.mouse.update(&app);
    model.ball.update(app.window_rect());

    if interesects(&model.ball, &model.mouse) {
        let dir = (model.ball.pos - model.mouse.pos).normalize();
        model.ball.add_force(model.mouse.force * dir);
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(SLATEGREY);

    // TODO: drawing logic goes here
    model.ball.draw(&app, &draw);
    draw.ellipse().xy(model.ball.pos).wh(vec2(5.0, 5.0)).color(BLACK);

    let r = model.mouse.radius;
    draw.ellipse().xy(model.mouse.pos).wh(vec2(2.0 * r, 2.0 * r));
    
    draw.to_frame(&app, &frame).unwrap();
    model.egui.draw_to_frame(&frame).unwrap();
}
