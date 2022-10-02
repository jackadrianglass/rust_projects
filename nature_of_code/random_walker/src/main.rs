use nannou::prelude::*;
use rand::{
    distributions::{Distribution, Standard, WeightedIndex},
    Rng,
};

#[derive(Clone, Copy)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Distribution<Dir> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Dir {
        let choices = [
            (Dir::Up, 1),
            (Dir::Down, 1),
            (Dir::Left, 1),
            (Dir::Right, 2),
        ];
        let dist = WeightedIndex::new(choices.iter().map(|v| v.1)).unwrap();
        choices[dist.sample(rng)].0
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

fn update(app: &App, model: &mut Model, _update: Update) {
    let mut rng = rand::thread_rng();
    let dir = app.mouse.position().normalize() * Vec2::new(5.0, 5.0);
    if dir.x.is_nan() || dir.y.is_nan() {
        let dir: Dir = rand::random();
        match dir {
            Dir::Up => model.y += 5.0,
            Dir::Down => model.y -= 5.0,
            Dir::Right => model.x += 5.0,
            Dir::Left => model.x -= 5.0,
        };
    } else {
        if rng.gen_bool(0.5) {
            model.x += dir.x;
            model.y += dir.y;
        } else {
            model.x -= dir.x;
            model.y -= dir.y;
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.rect()
        .x(model.x)
        .y(model.y)
        .w(5.0)
        .h(5.0)
        .color(SEAGREEN);
    draw.to_frame(app, &frame).unwrap();
}
