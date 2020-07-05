use anyhow::*;
use rustbox::{InitOptions, RustBox, Event, Key, RB_BOLD, Color};
use std::{thread, time::Duration};

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
}

impl Snake {
    fn new() -> Self {
        Self {
            position: Pos::new(0, 0),
            direction: Direction::Right,
        }
    }

    fn crawl(&mut self) {
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
        if self.position.x < 0 || self.position.y < 0 {
            return;
        }

        rb.print_char(
            self.position.x as usize,
            self.position.y as usize,
            RB_BOLD,
            Color::Green,
            Color::Default,
            'O'
        );
    }

    fn is_dead(&self) -> bool {
        todo!()
    }
}

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
