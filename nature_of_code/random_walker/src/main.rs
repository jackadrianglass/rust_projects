use nannou::prelude::*;
use rand::{distributions::{Distribution, Standard}, Rng};

enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Distribution<Dir> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Dir {
        match rng.gen_range(0..4) {
            0 => Dir::Up,
            1 => Dir::Right,
            2 => Dir::Down,
            3 => Dir::Left,
            val => panic!("Unexpected random value {}", val)
        }
    }
}

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

struct Model {
    x: f32,
    y: f32,
}

fn model(_app: &App) -> Model {
    Model { x: 0.0, y: 0.0 }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    let dir: Dir = rand::random();
    match dir {
        Dir::Up => model.y += 5.0,
        Dir::Down => model.y -= 5.0,
        Dir::Right => model.x += 5.0,
        Dir::Left => model.x -= 5.0,
    };
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.rect().x(model.x).y(model.y).w(5.0).h(5.0).color(SEAGREEN);
    draw.to_frame(app, &frame).unwrap();
}

