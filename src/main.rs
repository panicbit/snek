use anyhow::*;
use rustbox::{InitOptions, RustBox, Event, Key, RB_BOLD, RB_NORMAL, Color};
use std::{thread, time::Duration, collections::{BTreeSet, VecDeque}};
use rand::Rng;

const BASE_DELAY: f32 = 250.;
const ACCELERATION_BASE: f32 = 0.95;

fn main() -> Result<()> {
    let mut game = Game::new()?;

    game.run()?;

    Ok(())
}

struct Game {
    rb: RustBox,
    snake: Snake,
    pellets: BTreeSet<Pos>,
    score: usize,
    lost: bool,
}

impl Game {
    fn new() -> Result<Self> {
        let rb = RustBox::init(InitOptions::default())
            .context("Failed to initialize terminal")?;

        let mut game = Self {
            rb,
            snake: Snake::new(),
            pellets: BTreeSet::new(),
            score: 0,
            lost: false,
        };

        game.spawn_pellet();

        Ok(game)
    }

    fn spawn_pellet(&mut self) {
        let mut rng = rand::thread_rng();
        let width = rng.gen_range(0, self.rb.width()) as isize;
        let height = rng.gen_range(0, self.rb.height()) as isize;

        self.pellets.insert(Pos::new(width, height));
    }

    fn run(&mut self) -> Result<()> {
        loop {
            self.render();

            let mut delay = BASE_DELAY * ACCELERATION_BASE.powi(self.score as i32);

            if self.snake.direction().is_vertical() {
                delay *= 1.5;
            }

            let delay = Duration::from_millis(delay as u64);
            thread::sleep(delay);

            if self.run_logic_step()? == GameAction::Exit {
                return Ok(());
            }
        }
    }

    fn render(&self) {
        self.rb.clear();

        // Score
        self.rb.print(
            0,
            0,
            RB_BOLD,
            Color::Yellow,
            Color::Default,
            &format!("Score: {}", self.score),
        );

        // Game Over
        if self.lost {
            self.rb.print(
                0,
                1,
                RB_BOLD,
                Color::Red,
                Color::Default,
                "GAME OVER",
            );
        }

        // Pellets
        for pellet in &self.pellets {
            if pellet.x < 0 || pellet.y < 0 {
                continue;
            }

            self.rb.print_char(
                pellet.x as usize,
                pellet.y as usize,
                RB_BOLD,
                Color::Default,
                Color::Red,
                ' ',
            );
        }

        // Snake
        self.snake.render(&self.rb);

        self.rb.present();
    }

    fn run_logic_step(&mut self) -> Result<GameAction> {
        loop {
            let event = self.rb.peek_event(Duration::from_millis(1), false)?;

            match event {
                Event::KeyEvent(key) => match key {
                    Key::Up => self.snake.set_direction(Direction::Up),
                    Key::Down => self.snake.set_direction(Direction::Down),
                    Key::Left => self.snake.set_direction(Direction::Left),
                    Key::Right => self.snake.set_direction(Direction::Right),
                    Key::Esc | Key::Char('q') => return Ok(GameAction::Exit),
                    _ => continue
                },
                Event::NoEvent => break,
                _ => continue,
            }
        }

        if self.lost {
            return Ok(GameAction::KeepRunning);
        }

        let grow = self.pellets.remove(&self.snake.position);

        if grow {
            self.score += 1;
            self.spawn_pellet();
        }

        self.snake.crawl(grow);

        if self.snake.eating_itself() {
            self.snake.kill();
            self.lost = true;
        }

        Ok(GameAction::KeepRunning)
    }
}

#[derive(PartialEq)]
enum GameAction {
    Exit,
    KeepRunning,
}

struct Snake {
    position: Pos,
    direction: Direction,
    tail: VecDeque<Pos>,
    is_dead: bool,
}

impl Snake {
    fn new() -> Self {
        Self {
            position: Pos::new(10, 10),
            direction: Direction::Right,
            tail: VecDeque::from(vec![
                Pos::new(9, 10),
                Pos::new(8, 10),
                Pos::new(7, 10),
                Pos::new(6, 10),
                Pos::new(5, 10),
                Pos::new(4, 10),
            ]),
            is_dead: false,
        }
    }

    fn direction(&self) -> &Direction {
        &self.direction
    }

    fn crawl(&mut self, grow: bool) {
        if !grow {
            self.tail.pop_back();
        }

        self.tail.push_front(self.position);

        match self.direction {
            Direction::Up => self.position.y -= 1,
            Direction::Down => self.position.y += 1,
            Direction::Left => self.position.x -= 1,
            Direction::Right => self.position.x += 1,
        }
    }

    fn eating_itself(&self) -> bool {
        self.tail.contains(&self.position)
    }

    fn set_direction(&mut self, direction: Direction) {
        if self.is_dead || direction == self.direction.opposite(){
            return;
        }

        self.direction = direction;
    }

    fn kill(&mut self) {
        self.is_dead = true;
    }

    fn render(&self, rb: &RustBox) {
        // Tail
        for segment in &self.tail {
            if segment.x < 0 || segment.y < 0 {
                continue;
            }

            rb.print_char(
                segment.x as usize,
                segment.y as usize,
                RB_NORMAL,
                Color::Green,
                Color::Default,
                'o'
            );
        }

        // Head
        if self.position.x >= 0 && self.position.y >= 0 {
            let head_symbol = match self.direction {
                Direction::Up => '⮉',
                Direction::Down => '⮋',
                Direction::Left => '⮈',
                Direction::Right => '⮊',
            };

            let head_color = match self.is_dead {
                true => Color::Red,
                false => Color::Green,
            };

            rb.print_char(
                self.position.x as usize,
                self.position.y as usize,
                RB_BOLD,
                head_color,
                Color::Default,
                head_symbol,
            );
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Pos {
    x: isize,
    y: isize,
}

impl Pos {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn is_vertical(&self) -> bool {
        match self {
            Self::Up | Self::Down => true,
            _ => false,
        }
    }

    fn opposite(&self) -> Direction {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }
}