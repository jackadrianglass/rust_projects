use nannou::{
    color,
    noise::{utils::*, Fbm},
    prelude::*,
};

fn main() {
    nannou::sketch(view).run();
}

fn view(app: &App, frame: Frame) {
    if frame.nth() > 0 {
        return;
    }

    let window = app.window_rect();
    let fbm = Fbm::new();

    let map = PlaneMapBuilder::new(&fbm)
        .set_size(1024, 768)
        .set_x_bounds(-5.0, 5.0)
        .set_y_bounds(-5.0, 5.0)
        .build();


    let draw = app.draw();
    let d = 10;
    for x in (window.left() as i32..window.right() as i32).step_by(d) {
        for y in (window.bottom() as i32..window.top() as i32).step_by(d) {
            let x_val = (x as f32 + window.right()) as usize;
            let y_val = (y as f32 + window.top()) as usize;
            draw.rect()
                .xy(Vec2::new(x as f32, y as f32))
                .wh(Vec2::new(d as f32, d as f32))
                .color(color::gray(
                    (map.get_value(x_val, y_val) + 1.0) / 2.0,
                ));
        }
    }
    draw.to_frame(app, &frame).unwrap();
}
