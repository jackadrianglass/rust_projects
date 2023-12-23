use na::vector;
use nalgebra as na;

pub enum Dir {
    Pos,
    Neg,
}

pub fn dragon_curve(a: na::Point3<f32>, b: na::Point3<f32>, depth: i32) -> Vec<na::Point3<f32>> {
    let mut points = Vec::new();
    points.push(a);
    dragon_curve_algo(&mut points, a, b, depth, Dir::Neg);
    points.push(b);

    points
}

/*
auto fractals::angle_from_x_axis(glm::vec3 vec) -> float
{
  static const auto rad_180 = deg_to_rad(180);
  static const auto rad_360 = deg_to_rad(360);
  const auto intermediate = std::abs(std::atan(vec.y / vec.x));
  // Go by quadrants
  if(vec.x >= 0 && vec.y >= 0){
    return intermediate;
  }else if(vec.x < 0 && vec.y >= 0){
    return rad_180 - intermediate;
  }else if(vec.x < 0 && vec.y < 0){
    return rad_180 + intermediate;
  } else {
    return rad_360 - intermediate;
  }
}
*/

fn angle_from_x_axis(vec: na::Point3<f32>) -> f32 {
    let rad_180 = 180.0_f32.to_radians();
    let rad_360 = 360.0_f32.to_radians();
    let intermediate = (vec.y / vec.x).atan().abs();

    // Go by quadrants
    if vec.x >= 0.0 && vec.y >= 0.0 {
        intermediate
    } else if vec.x < 0.0 && vec.y >= 0.0 {
        rad_180 - intermediate
    } else if vec.x < 0.0 && vec.y < 0.0 {
        rad_180 + intermediate
    } else {
        rad_360 - intermediate
    }
}

fn dragon_curve_algo(
    points: &mut Vec<na::Point3<f32>>,
    a: na::Point3<f32>,
    b: na::Point3<f32>,
    depth: i32,
    direction: Dir,
) {
    /*
      static const auto neg_dir = deg_to_rad(315);
      static const auto pos_dir = deg_to_rad(45);

      if(iter <= 0) return;
      const auto AB = B - A;
      const auto theta = angle_from_x_axis(AB);
      const auto calc_c = [](auto& A, auto& B, auto theta, auto rads){
           const auto length_AC = glm::distance(A, B) * cos(rads);
           return A + glm::vec3{
             length_AC * cos(theta + rads),
             length_AC * sin(theta + rads),
             0.0f
           };
      };
      glm::vec3 C;
      if(direction == Dir::Pos){
        C = calc_c(A, B, theta, pos_dir);
      } else {
        C = calc_c(A, B, theta, neg_dir);
      }
    }
        */
    let neg_dir = 315.0_f32.to_radians();
    let pos_dir = 45.0_f32.to_radians();

    if depth <= 0 {
        return;
    }

    let ab = b - a;
    let theta = angle_from_x_axis(ab.into());
    let calc_c = |a: na::Point3<f32>, b: na::Point3<f32>, theta: f32, rads: f32| {
        let length_ab = na::distance(&a, &b) * rads.cos();
        a + vector![
            length_ab * (theta + rads).cos(),
            length_ab * (theta + rads).sin(),
            0.0
        ]
    };

    let c = match direction {
        Dir::Pos => calc_c(a, b, theta, pos_dir),
        Dir::Neg => calc_c(a, b, theta, neg_dir),
    };

    //caller adds point A
    dragon_curve_algo(points, a, c, depth - 1, Dir::Neg);
    points.push(c);
    dragon_curve_algo(points, c, b, depth - 1, Dir::Pos);
    //caller adds point B
}
