use std::time::{UNIX_EPOCH, SystemTime};
use nannou::{
    color,
    noise::{NoiseFn, Perlin, Seedable},
    prelude::*,
};

fn main() {
    nannou::sketch(view).run();
}

fn view(app: &App, frame: Frame) {
    if frame.nth() > 0 {
        return;
    }

    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    let perlin = Perlin::new().set_seed(now as u32);
    let window = app.window_rect();

    let draw = app.draw();
    let d = 5;
    for x in (window.left() as i32..window.right() as i32).step_by(d) {
        for y in (window.bottom() as i32..window.top() as i32).step_by(d) {
            draw.rect()
                .xy(Vec2::new(x as f32, y as f32))
                .wh(Vec2::new(d as f32, d as f32))
                .color(color::gray(255.0 * perlin.get([(x as f32 / window.right()) as f64, (y as f32 / window.top()) as f64])));
        }
    }
    draw.to_frame(app, &frame).unwrap();
}
