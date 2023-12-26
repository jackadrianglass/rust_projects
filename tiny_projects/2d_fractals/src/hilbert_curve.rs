use nalgebra as na;

pub fn hilbert_curve(depth: i32) -> Vec<na::Point3<f32>> {
    let n = 2_u32.pow(depth as u32);
    let total_points = n * n;
    let line_length = 2.0 / n as f32;

    let transformation = na::Vector3::new(line_length / 2.0 - 1.0, line_length / 2.0 - 1.0, 0.0);

    (0..total_points)
        .map(|i| algo(i, n) * line_length + transformation)
        .collect()
}

fn algo(idx: u32, n: u32) -> na::Point3<f32> {
    let points = [
        na::Point3::new(0.0, 0.0, 0.0),
        na::Point3::new(0.0, 1.0, 0.0),
        na::Point3::new(1.0, 1.0, 0.0),
        na::Point3::new(1.0, 0.0, 0.0),
    ];

    let mut result = points[(idx & 3) as usize];
    let mut idx = idx >> 2;

    let mut i = 4;
    loop {
        if i > n {
            break;
        }
        let i2 = i as f32 / 2.0;
        match idx & 3 {
            0 => {
                let tmp = result.x;
                result.x = result.y;
                result.y = tmp;
            }
            1 => result.y += i2,
            2 => {
                result.x += i2;
                result.y += i2;
            }
            3 => {
                let tmp = result.y;
                result.y = i2 - 1.0 - result.x;
                result.x = (i2 - 1.0 - tmp) + i2;
            }
            _ => {}
        }
        idx >>= 2;
        i *= 2;
    }

    result
}
