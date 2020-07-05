use anyhow::*;
use rustbox::{InitOptions, RustBox, Event, Key, RB_BOLD, RB_NORMAL, Color};
use std::{thread, time::Duration, collections::VecDeque};

fn main() -> Result<()> {
    let mut game = Game::new()?;

    game.run()?;

    Ok(())
}

struct Game {
    rb: RustBox,
    snake: Snake,
}

impl Game {
    fn new() -> Result<Self> {
        let rb = RustBox::init(InitOptions::default())
            .context("Failed to initialize terminal")?;
        
        Ok(Self {
            rb,
            snake: Snake::new(),
        })
    }

    fn run(&mut self) -> Result<()> {
        loop {
            self.render();

            if self.run_logic_step()? == GameAction::Exit {
                return Ok(());
            }
        }
    }

    fn render(&self) {
        self.rb.clear();
        self.snake.render(&self.rb);
        self.rb.present();
        thread::sleep(Duration::from_millis(500));
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

        self.snake.crawl();

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
        }
    }

    fn crawl(&mut self) {
        self.tail.pop_back();
        self.tail.push_front(self.position);

        match self.direction {
            Direction::Up => self.position.y -= 1,
            Direction::Down => self.position.y += 1,
            Direction::Left => self.position.x -= 1,
            Direction::Right => self.position.x += 1,
        }
    }

    fn set_direction(&mut self, direction: Direction) {
        self.direction = direction;
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

            rb.print_char(
                self.position.x as usize,
                self.position.y as usize,
                RB_BOLD,
                Color::Green,
                Color::Default,
                head_symbol,
            );
        }
    }

    fn is_dead(&self) -> bool {
        todo!()
    }
}

#[derive(Copy, Clone)]
struct Pos {
    x: isize,
    y: isize,
}

impl Pos {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}
