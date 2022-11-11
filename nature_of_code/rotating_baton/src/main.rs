use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};

struct Model {
    egui: Egui,
    min_l: f32,
    max_l: f32,
    delta_l: f32,
    l_rate: f32,
    rvel: f32,
    angle: f32,
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
        min_l: 30.0,
        max_l: 200.0,
        delta_l: 0.0,
        l_rate: 0.01,
        rvel: 0.5,
        angle: 0.0,
    }
}

fn update(_app: &App, model: &mut Model, update: Update) {
    let egui = &mut model.egui;

    egui.set_elapsed_time(update.since_start);
    let ctx = egui.begin_frame();

    egui::Window::new("Settings").show(&ctx, |ui| {
        ui.add(egui::Slider::new(&mut model.l_rate, 0.001..=2.0).text("Change in baton length speed"));
        ui.add(egui::Slider::new(&mut model.rvel, 0.001..=2.0).text("Rotational Velocity"));
    });

    model.angle += model.rvel;
    model.delta_l += model.l_rate;
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(SLATEGREY);

    let length = model.delta_l.sin() * model.max_l + model.min_l;
    let start = vec2(model.angle.cos(), model.angle.sin()) * length;
    let end = start * -1.0;

    draw.line().weight(5.0).start(start).end(end).color(BLACK);
    draw.ellipse().xy(start).wh(vec2(25.0, 25.0));
    draw.ellipse().xy(end).wh(vec2(25.0, 25.0));

    draw.to_frame(&app, &frame).unwrap();
    model.egui.draw_to_frame(&frame).unwrap();
}
