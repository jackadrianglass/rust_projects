use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};

struct Model {
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
    }
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
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(SLATEGREY);

    // TODO: drawing logic goes here
    
    draw.to_frame(&app, &frame).unwrap();
    model.egui.draw_to_frame(&frame).unwrap();
}
