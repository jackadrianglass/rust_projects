use nannou::prelude::*;

struct Model {
    pos: Vec2,
    vel: Vec2,
    acc: Vec2,
    radius: f32,
}

fn main() {
    nannou::app(model).event(update).simple_window(view).run();
}

fn model(_app: &App) -> Model {
    Model {
        pos: Vec2::new(0.0, 0.0),
        vel: Vec2::new(150.0, 175.0),
        acc: Vec2::new(0.0, -500.0),
        radius: 50.0,
    }
}

fn update(app: &App, model: &mut Model, _event: Event) {
    let delta = app.duration.since_prev_update.as_secs_f32();
    model.pos += model.vel * delta;
    model.vel += model.acc * delta;

    let win = app.window_rect();
    if model.pos.x + model.radius >= win.right() {
        model.pos.x = win.right() - model.radius;
        model.vel.x *= -1.0;
    } else if model.pos.x - model.radius <= win.left() {
        model.pos.x = win.left() + model.radius;
        model.vel.x *= -1.0;
    }

    if model.pos.y + model.radius >= win.top() {
        model.pos.y = win.top() - model.radius;
        model.vel.y *= -1.0;
    } else if model.pos.y - model.radius <= win.bottom() {
        model.pos.y = win.bottom() + model.radius;
        model.vel.y *= -1.0;
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(SLATEGREY);

    draw.ellipse()
        .xy(model.pos)
        .wh(Vec2::new(2.0 * model.radius, 2.0 * model.radius))
        .color(WHITESMOKE);

    draw.to_frame(&app, &frame).unwrap();
}
