use anyhow::*;
use rustbox::{InitOptions, RustBox, Event, Key, RB_BOLD, Color};
use std::{thread, time::Duration, collections::BTreeSet};
use rand::Rng;
use crate::{BASE_DELAY, Pos, ACCELERATION_BASE, GameAction, Direction, Snake};

pub struct Game {
    rb: RustBox,
    snake: Snake,
    pellets: BTreeSet<Pos>,
    score: usize,
    lost: bool,
}

impl Game {
    pub fn new() -> Result<Self> {
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

    pub fn run(&mut self) -> Result<()> {
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

        let grow = self.pellets.remove(&self.snake.position());

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
