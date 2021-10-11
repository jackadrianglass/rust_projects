use ggez::conf::{FullscreenType, NumSamples, WindowMode, WindowSetup};
use ggez::event::{self, KeyCode};
use ggez::graphics::{self, Mesh, Text};
use ggez::{ContextBuilder, GameResult};
use glam::*;
use rand::{self, Rng};

#[derive(Clone, Copy)]
enum Dir {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}

impl Dir {
    fn value(&self) -> Vec2 {
        match *self {
            Dir::UP => Vec2::new(0.0, -1.0),
            Dir::RIGHT => Vec2::new(1.0, 0.0),
            Dir::DOWN => Vec2::new(0.0, 1.0),
            Dir::LEFT => Vec2::new(-1.0, 0.0),
        }
    }
    fn opposite(&self) -> Vec2 {
        self.value() * -1.0
    }
}

#[derive(Clone, Copy)]
struct Point {
    pos: Vec2,
    dir: Dir,
}

#[derive(Clone)]
struct Node {
    pos: Vec2,
    dir: Dir,
    next_dir_change: Option<Point>,
}

impl Node {
    fn update(&mut self, speed: f32) -> Option<Point> {
        self.pos += self.dir.value() * speed;
        if let Some(point) = self.next_dir_change {
            match self.dir {
                Dir::UP => {
                    if self.pos.y <= point.pos.y {
                        self.pos.y = point.pos.y;
                        self.dir = point.dir;
                        self.next_dir_change = None;
                        return Some(point);
                    }
                }
                Dir::DOWN => {
                    if self.pos.y >= point.pos.y {
                        self.pos.y = point.pos.y;
                        self.dir = point.dir;
                        self.next_dir_change = None;
                        return Some(point);
                    }
                }
                Dir::LEFT => {
                    if self.pos.x <= point.pos.x {
                        self.pos.x = point.pos.x;
                        self.dir = point.dir;
                        self.next_dir_change = None;
                        return Some(point);
                    }
                }
                Dir::RIGHT => {
                    if self.pos.x >= point.pos.x {
                        self.pos.x = point.pos.x;
                        self.dir = point.dir;
                        self.next_dir_change = None;
                        return Some(point);
                    }
                }
            }
        }
        None
    }

    fn change_dir(&mut self, dir: Dir) -> bool {
        match self.dir {
            Dir::UP | Dir::DOWN => match dir {
                Dir::LEFT | Dir::RIGHT => {
                    self.dir = dir;
                    true
                }
                _ => false,
            },
            Dir::LEFT | Dir::RIGHT => match dir {
                Dir::UP | Dir::DOWN => {
                    self.dir = dir;
                    true
                }
                _ => false,
            },
        }
    }
}

/*------------------------------------------------------------------------
| Game State
\*-----------------------------------------------------------------------*/
struct SnakeState {
    window: Vec2,
    radius: f32,
    speed: f32,
    head: Node,
    tail: Vec<Node>,
    candy: Vec2,
    score: i32,
    running: bool,
}

impl SnakeState {
    fn new(window: Vec2, speed: f32, radius: f32) -> Self {
        let increment = Dir::LEFT.value() * (1.75 * radius);
        let start_pos = window / 2.0;
        let mut rng = rand::thread_rng();
        Self {
            window: window.clone(),
            speed,
            radius,
            head: Node {
                pos: start_pos.clone(),
                dir: Dir::RIGHT,
                next_dir_change: None,
            },
            tail: vec![
                Node {
                    pos: start_pos + increment,
                    dir: Dir::RIGHT,
                    next_dir_change: None,
                },
                Node {
                    pos: start_pos + 2.0 * increment,
                    dir: Dir::RIGHT,
                    next_dir_change: None,
                },
            ],
            candy: Vec2::new(
                rng.gen_range(radius..window.x),
                rng.gen_range(radius..window.y),
            ),
            score: 0,
            running: true,
        }
    }

    fn restart(&mut self) {
        let start_pos = self.window / 2.0;
        let increment = Dir::LEFT.value() * (1.75 * self.radius);
        self.head = Node {
            pos: start_pos.clone(),
            dir: Dir::RIGHT,
            next_dir_change: None,
        };
        self.tail = vec![
            Node {
                pos: start_pos + increment,
                dir: Dir::RIGHT,
                next_dir_change: None,
            },
            Node {
                pos: start_pos + 2.0 * increment,
                dir: Dir::RIGHT,
                next_dir_change: None,
            },
        ];

        let mut rng = rand::thread_rng();
        self.candy = Vec2::new(
            rng.gen_range(self.radius..self.window.x),
            rng.gen_range(self.radius..self.window.y),
        );

        self.score = 0;
        self.running = true;
    }

    fn change_dir(&mut self, dir: Dir) {
        if self.tail[0].next_dir_change.is_some() {
            return;
        }

        if self.head.change_dir(dir) {
            self.tail[0].next_dir_change = Some(Point {
                pos: self.head.pos.clone(),
                dir: self.head.dir.clone(),
            });
        }
    }

    fn update_pos(&mut self) {
        self.head.update(self.speed);

        let mut next_change = None;
        for idx in 0..self.tail.len() {
            let tmp = if let Some(point) = self.tail[idx].update(self.speed) {
                Some(point)
            } else {
                None
            };

            match next_change {
                Some(_) => {
                    self.tail[idx].next_dir_change = next_change;
                }
                None => {}
            }

            next_change = tmp;
        }
    }

    fn check_candy_collision(&mut self) {
        if self.head.pos.distance(self.candy) >= self.radius * 2.0 {
            // didn't collide with the candy
            return;
        }

        self.score += 1;

        if let Some(last) = self.tail.last() {
            let last = last.clone(); // borrow checker doesn't like me if I don't add this
            let increment = last.dir.opposite() * (1.75 * self.radius);
            let mut rng = rand::thread_rng();

            self.tail.push(Node {
                pos: last.pos + increment,
                dir: last.dir,
                next_dir_change: None,
            });

            self.candy = Vec2::new(
                rng.gen_range(self.radius..self.window.x),
                rng.gen_range(self.radius..self.window.y),
            );
        }
    }

    fn check_tail_collisions(&mut self) {
        for node in self.tail.iter().skip(1) {
            if self.head.pos.distance(node.pos) < self.radius * 2.0 {
                self.running = false;
            }
        }
    }

    fn check_wall_collision(&mut self) {
        if self.head.pos.x - self.radius <= 0.0
            || self.head.pos.x + self.radius >= self.window.x
            || self.head.pos.y - self.radius <= 0.0
            || self.head.pos.y + self.radius >= self.window.y
        {
            self.running = false;
        }
    }
}

impl event::EventHandler<ggez::GameError> for SnakeState {
    fn key_down_event(
        &mut self,
        _ctx: &mut ggez::Context,
        keycode: event::KeyCode,
        _keymods: event::KeyMods,
        repeat: bool,
    ) {
        if repeat {
            return;
        }

        match keycode {
            KeyCode::W => self.change_dir(Dir::UP),
            KeyCode::D => self.change_dir(Dir::RIGHT),
            KeyCode::S => self.change_dir(Dir::DOWN),
            KeyCode::A => self.change_dir(Dir::LEFT),
            KeyCode::R => self.restart(),
            _ => {}
        }
    }

    fn key_up_event(
        &mut self,
        _ctx: &mut ggez::Context,
        _keycode: event::KeyCode,
        _keymods: event::KeyMods,
    ) {
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> Result<(), ggez::GameError> {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        let head = Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            Vec2::new(0.0, 0.0),
            self.radius,
            1.0,
            graphics::Color::GREEN,
        )?;
        let tail = Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            Vec2::new(0.0, 0.0),
            self.radius,
            1.0,
            graphics::Color::YELLOW,
        )?;
        let candy = Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            Vec2::new(0.0, 0.0),
            self.radius,
            1.0,
            graphics::Color::RED,
        )?;

        let score = Text::new(format!("{}", self.score));

        graphics::draw(ctx, &head, (self.head.pos.clone(),))?;
        for node in &self.tail {
            graphics::draw(ctx, &tail, (node.pos.clone(),))?;
        }
        graphics::draw(ctx, &candy, (self.candy.clone(),))?;
        graphics::draw(ctx, &score, (Vec2::new(self.window.x / 2.0, 25.0),))?;

        if !self.running {
            let gameover = Text::new("GAME OVER!!");
            graphics::draw(ctx, &gameover, (self.window / 2.0,))?;
        }
        graphics::present(ctx)?;
        Ok(())
    }

    fn update(&mut self, _ctx: &mut ggez::Context) -> Result<(), ggez::GameError> {
        if !self.running {
            return Ok(());
        }

        self.update_pos();
        self.check_candy_collision();
        self.check_tail_collisions();
        self.check_wall_collision();
        Ok(())
    }
}

/*------------------------------------------------------------------------
| Main
\*-----------------------------------------------------------------------*/
fn main() -> GameResult {
    let window = Vec2::new(800.0, 600.0);
    let speed = 1.75;
    let radius = 10.0;
    let cb = ContextBuilder::new("Snake", "Jack Glass")
        .window_setup(WindowSetup {
            title: "Snake!".to_owned(),
            samples: NumSamples::One,
            vsync: true,
            icon: "".to_owned(),
            srgb: true,
        })
        .window_mode(WindowMode {
            width: window.x,
            height: window.y,
            maximized: false,
            fullscreen_type: FullscreenType::Windowed,
            borderless: false,
            min_width: 0.0,
            min_height: 0.0,
            max_width: 0.0,
            max_height: 0.0,
            resizable: false,
            visible: true,
            resize_on_scale_factor_change: true,
        });

    let (ctx, event_loop) = cb.build()?;
    let state = SnakeState::new(window, speed, radius);
    event::run(ctx, event_loop, state);
}
