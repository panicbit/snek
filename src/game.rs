use anyhow::*;
use rustbox::{InitOptions, RustBox, Event, Key, RB_BOLD, Color};
use std::{thread, time::Duration, collections::BTreeSet};
use rand::Rng;
use crate::{Pos, ACCELERATION_BASE, GameAction, Direction, Snake, FIELD_TRAVERSAL_TIME_MILLIS};

pub struct Game {
    rb: RustBox,
    snake: Snake,
    pellets: BTreeSet<Pos>,
    score: usize,
    lost: bool,
    paused: bool,
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
            paused: false,
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

            let field_size = match self.snake.direction().is_vertical() {
                false => self.rb.width() as f32,
                true => self.rb.height() as f32 * 1.5,
            };
            let delay = FIELD_TRAVERSAL_TIME_MILLIS / field_size * ACCELERATION_BASE.powi(self.score as i32);
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
        let mut next_direction = *self.snake.direction();

        loop {
            let event = self.rb.peek_event(Duration::from_millis(1), false)?;

            match event {
                Event::KeyEvent(key) => match key {
                    Key::Up => next_direction = Direction::Up,
                    Key::Down => next_direction = Direction::Down,
                    Key::Left => next_direction = Direction::Left,
                    Key::Right => next_direction = Direction::Right,
                    Key::Esc | Key::Char('q') => return Ok(GameAction::Exit),
                    Key::Char(' ') => self.paused = !self.paused,
                    _ => continue
                },
                Event::NoEvent => break,
                _ => continue,
            }
        }

        if self.paused {
            return Ok(GameAction::KeepRunning);
        }

        self.snake.set_direction(next_direction);

        if self.lost {
            return Ok(GameAction::KeepRunning);
        }

        let grow = self.pellets.remove(&self.snake.position());

        if grow {
            self.score += 1;
            self.spawn_pellet();
        }

        self.snake.crawl(grow);

        // Game Over condition
        if self.snake.eating_itself() || self.snake_outside_bounds() {
            self.snake.kill();
            self.lost = true;
        }

        Ok(GameAction::KeepRunning)
    }

    fn snake_outside_bounds(&self) -> bool {
        let width = self.rb.width() as isize;
        let height = self.rb.height() as isize;
        let position = self.snake.position();

           position.x < 0 || position.x >= width
        || position.y < 0 || position.y >= height
    }
}
