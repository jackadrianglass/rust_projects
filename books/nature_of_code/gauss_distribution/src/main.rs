use nannou::prelude::*;
use nannou::color;
use rand::prelude::*;
use rand_distr::Normal;

fn main() {
    nannou::sketch(view).run();
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();
    let mut rng = rand::thread_rng();
    let normal = Normal::new(0.0, 100.0).unwrap();
    let n_color = Normal::new(125.0, 125.0).unwrap();

    let pos = Vec2::new(normal.sample(&mut rng), normal.sample(&mut rng));
    let r = (255.0 * n_color.sample(&mut rng)) as u8;
    let g = (255.0 * n_color.sample(&mut rng)) as u8;
    let b = (255.0 * n_color.sample(&mut rng)) as u8;
    draw.ellipse().xy(pos).color(color::rgba8(r, g, b, 100));
    draw.to_frame(app, &frame).unwrap();
}
