use nannou::noise::{BasicMulti, NoiseFn};
use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};

const NOISE_STEP: f64 = 500.0;

struct Model {
    noise: BasicMulti,
    points: Vec<Vec2>,
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

    Model {
        egui,
        points: Vec::new(),
        noise: BasicMulti::new(),
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

    // TODO: update logic goes here

    // if the index into the get method is an integer, then it will
    // always return 0
    let x = app.elapsed_frames() as f64 / NOISE_STEP;

    let win = app.window_rect();
    let y = model.noise.get([x, 0.0]);
    let mapped_y = map_range(y, -1.0, 1.0, win.top(), win.bottom());

    model.points.push(vec2(x as f32, mapped_y as f32));
}

fn view(app: &App, model: &Model, frame: Frame) {
    let win = app.window_rect();
    let draw = app.draw();
    draw.background().color(SLATEGREY);

    // TODO: drawing logic goes here
    draw.line()
        .start(vec2(win.left(), 0.0))
        .end(vec2(win.right(), 0.0))
        .weight(2.0)
        .color(WHITESMOKE);

    draw.polyline()
        .x(0.0 - model.points.len() as f32)
        .points(
            model
                .points
                .iter()
                .cloned()
                .enumerate()
                .map(|(idx, mut p)| {
                    p.x = idx as f32;
                    p
                })
                .collect::<Vec<Vec2>>(),
        )
        .color(WHITE);

    draw.ellipse()
        .x(0.0)
        .y(model.points.last().unwrap().y)
        .wh(vec2(25.0, 25.0))
        .color(rgba(0.1, 0.1, 0.9, 0.9))
        .stroke_weight(2.0)
        .stroke_color(WHITE);

    draw.to_frame(&app, &frame).unwrap();
    model.egui.draw_to_frame(&frame).unwrap();
}
