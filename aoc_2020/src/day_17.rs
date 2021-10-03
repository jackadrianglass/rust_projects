#[derive(Debug, PartialEq)]
struct Dim {
    x: i32,
    y: i32,
    z: i32,
}

fn find_max_coords(pocket_dim: &[Dim]) -> Dim {
    let x = pocket_dim.iter().max_by(|first, second| first.x.cmp(&second.x)).unwrap();
    let y = pocket_dim.iter().max_by(|first, second| first.y.cmp(&second.y)).unwrap();
    let z = pocket_dim.iter().max_by(|first, second| first.z.cmp(&second.z)).unwrap();
    Dim{x: x.x, y: y.y, z: z.z}
}

fn find_min_coords(pocket_dim: &[Dim]) -> Dim {
    let x = pocket_dim.iter().min_by(|first, second| first.x.cmp(&second.x)).unwrap();
    let y = pocket_dim.iter().min_by(|first, second| first.y.cmp(&second.y)).unwrap();
    let z = pocket_dim.iter().min_by(|first, second| first.z.cmp(&second.z)).unwrap();
    Dim{x: x.x, y: y.y, z: z.z}
}

fn becomes_active(pocket_dim: &[Dim], current: &Dim, active: bool) -> bool {
    let mut count = 0;
    for x in -1..1 {
        for y in -1..1 {
            for z in -1..1 {
                if x == 0 && y == 0 && z == 0 {continue;}
                let check = Dim{x: current.x + x, y: current.y + y, z: current.z + z};
                if pocket_dim.contains(&check) {
                    count += 1;
                }
            }
        }
    }
    if active && (count == 2 || count == 3) {
        return true;
    } else if !active && count == 3 {
        return true;
    }
    false
}

fn main() {
    println!("Hello world");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_something() {

    }
}
