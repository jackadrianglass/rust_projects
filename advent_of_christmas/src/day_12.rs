use std::fs;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Dir {
    N,
    S,
    E,
    W,
    F,
    R,
    L
}

#[derive(Debug, PartialEq)]
struct Inst {
    dir: Dir,
    amount: i32
}

impl Inst {
    fn from_str(src: &str) -> Self
    {
        let dir = match &src[..1]{
            "N" => {Dir::N}
            "S" => {Dir::S}
            "E" => {Dir::E}
            "W" => {Dir::W}
            "F" => {Dir::F}
            "R" => {Dir::R}
            "L" => {Dir::L}
            _ => {panic!("Invalid dir");}
        };

        Inst{dir: dir, amount: src[1..].parse().unwrap()}
    }
}

fn parse_file(contents: &str) -> Vec<Inst> {
    contents.lines().map(|val| Inst::from_str(val)).collect()
}

#[derive(Clone)]
struct Pos {
    x: i32,
    y: i32,
}

struct Ship {
    pos: Pos,
    wpos: Pos,
    dir: Dir,
}

impl Ship {
    fn new() -> Self {
        Ship{pos: Pos{x: 0, y: 0}, wpos: Pos{x: 10, y: 1}, dir: Dir::E}
    }

    fn process_inst(&mut self, insts: &[Inst]) {
        for inst in insts {
            self.move_by(inst);
        }
    }

    fn process_waypoint_inst(&mut self, insts: &[Inst]) {
        for inst in insts {
            self.move_by_waypoint(inst);
        }
    }

    fn move_by_waypoint(&mut self, inst: &Inst) {
        match inst.dir {
            Dir::N => {self.wpos.y += inst.amount;}
            Dir::S => {self.wpos.y -= inst.amount;}
            Dir::E => {self.wpos.x += inst.amount;}
            Dir::W => {self.wpos.x -= inst.amount;}
            Dir::F => {
                self.pos.x += self.wpos.x * inst.amount;
                self.pos.y += self.wpos.y * inst.amount;
            }
            Dir::R => {self.rotate_waypoint(inst.amount);}
            Dir::L => {self.rotate_waypoint(-1 * inst.amount);}
        }
    }

    fn rotate_waypoint(&mut self, amount: i32) {
        if amount % 90 != 0 {
            panic!("uh oh");
        }

        let quart_turns = amount / 90;
        for _ in 0..quart_turns.abs() {
            if quart_turns < 0 {
                let tmp = self.wpos.clone();
                self.wpos.x = -1 * tmp.y;
                self.wpos.y = tmp.x;
            } else {
                let tmp = self.wpos.clone();
                self.wpos.x = tmp.y;
                self.wpos.y = -1 * tmp.x;
            }
        }
    }

    fn move_by(&mut self, inst: &Inst) {
        match inst.dir {
            Dir::N => {self.pos.y += inst.amount;}
            Dir::S => {self.pos.y -= inst.amount;}
            Dir::E => {self.pos.x += inst.amount;}
            Dir::W => {self.pos.x -= inst.amount;}
            Dir::F => {self.move_by(&Inst{dir: self.dir, amount: inst.amount});}
            Dir::R => {self.rotate(inst.amount);}
            Dir::L => {self.rotate(-1 * inst.amount);}
        }
    }

    fn rotate(&mut self, amount: i32) {
        if amount % 90 != 0 {
            panic!("uh oh");
        }

        let quart_turns = amount / 90;
        for _ in 0..quart_turns.abs() {
            match self.dir {
                Dir::N => {if quart_turns < 0 {self.dir = Dir::W;} else {self.dir = Dir::E;}}
                Dir::E => {if quart_turns < 0 {self.dir = Dir::N;} else {self.dir = Dir::S;}}
                Dir::S => {if quart_turns < 0 {self.dir = Dir::E;} else {self.dir = Dir::W;}}
                Dir::W => {if quart_turns < 0 {self.dir = Dir::S;} else {self.dir = Dir::N;}}
                _ => {}
            }
        }
    }

    fn manhattan_dist(&self) -> i32 {
        self.pos.x.abs() + self.pos.y.abs()
    }
}

fn main() {
    let contents = fs::read_to_string("day_12.txt")
                            .expect("Unable to read file");

    let insts = parse_file(&contents);
    let mut ship = Ship::new();
    ship.process_inst(&insts);

    let dist = ship.manhattan_dist();
    println!("Manhattan distance is {}", dist);

    //part 2
    let mut ship = Ship::new();
    ship.process_waypoint_inst(&insts);

    let dist = ship.manhattan_dist();
    println!("Manhattan distance is {}", dist);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let expect = Inst{dir: Dir::F, amount: 90};
        assert_eq!(Inst::from_str("F90"), expect);
    }

    #[test]
    fn test_parse_many() {
        let expect = vec!{
            Inst{dir: Dir::F, amount: 10},
            Inst{dir: Dir::N, amount: 3},
            Inst{dir: Dir::F, amount: 7},
            Inst{dir: Dir::R, amount: 90},
            Inst{dir: Dir::F, amount: 11},
        };
        let input = "F10\nN3\nF7\nR90\nF11";

        assert_eq!(parse_file(input), expect);
    }

    #[test]
    fn test_move() {
        let input = "F10\nN3\nF7\nR90\nF11";
        let input = parse_file(input);
        let mut ship = Ship::new();
        ship.process_inst(&input);

        assert_eq!(ship.manhattan_dist(), 25);
    }

    #[test]
    fn test_movement() {
        let mut ship = Ship::new();
        ship.move_by(&Inst{dir: Dir::N, amount: 10});
        assert_eq!(ship.pos.x, 0);
        assert_eq!(ship.pos.y, 10);

        ship.move_by(&Inst{dir: Dir::R, amount: 90});
        assert_eq!(ship.dir, Dir::S);

        ship.move_by(&Inst{dir: Dir::L, amount: 270});
        assert_eq!(ship.dir, Dir::W);
    }
    
    #[test]
    fn test_waypoint_movement() {
        let mut ship = Ship::new();
        ship.move_by_waypoint(&Inst{dir: Dir::F, amount: 10});
        assert_eq!(ship.pos.x, 100);
        assert_eq!(ship.pos.y, 10);

        ship.move_by_waypoint(&Inst{dir: Dir::N, amount: 3});
        assert_eq!(ship.wpos.x, 10);
        assert_eq!(ship.wpos.y, 4);

        ship.move_by_waypoint(&Inst{dir: Dir::F, amount: 7});
        assert_eq!(ship.pos.x, 170);
        assert_eq!(ship.pos.y, 38);

        ship.move_by_waypoint(&Inst{dir: Dir::R, amount: 90});
        assert_eq!(ship.wpos.x, 4);
        assert_eq!(ship.wpos.y, -10);

        ship.move_by_waypoint(&Inst{dir: Dir::F, amount: 11});
        assert_eq!(ship.pos.x, 214);
        assert_eq!(ship.pos.y, -72);
    }
}
