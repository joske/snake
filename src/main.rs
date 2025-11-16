use crossterm::{
    cursor::{self, Hide, Show},
    event::{poll, read, Event, KeyCode},
    execute,
    style::{self, Stylize},
    terminal::{self, disable_raw_mode, enable_raw_mode},
    ExecutableCommand, QueueableCommand,
};
use rand::Rng;
use std::{
    collections::LinkedList,
    io::{stdout, Write},
    process::exit,
    time::Duration,
};

const COLS: u16 = 30;
const ROWS: u16 = 20;
const DELAY: u64 = 250;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Location {
    x: u16,
    y: u16,
}
impl Location {
    fn random() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            x: rng.gen_range(1..COLS),
            y: rng.gen_range(1..ROWS),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Segment {
    pos: Location,
}

struct Food {
    pos: Location,
}

#[derive(Debug)]
struct Snake {
    segments: LinkedList<Segment>,
    dir: Direction,
}

impl Snake {
   pub fn new() -> Self {
        Snake {
            segments: LinkedList::from([
                Segment {
                    pos: Location { x: 3, y: 1 },
                },
                Segment {
                    pos: Location { x: 2, y: 1 },
                },
                Segment {
                    pos: Location { x: 1, y: 1 },
                },
            ]),
            dir: Direction::Right,
        }
    }

    fn update_snake(&mut self, tick: u64) {
        let head = self.segments.front().unwrap(); // there's always a head

        let (x, y) = match self.dir {
            Direction::Up => (head.pos.x, head.pos.y - 1),
            Direction::Down => (head.pos.x, head.pos.y + 1),
            Direction::Left => (head.pos.x - 1, head.pos.y),
            Direction::Right => (head.pos.x + 1, head.pos.y),
        };
        if x == 0 || x >= COLS || y == 0 || y >= ROWS || self.hit_tail(x, y) {
            // kapoet
            game_over();
        }
        let new_head = Segment {
            pos: Location { x, y },
        };
        self.segments.push_front(new_head);
        if tick % 10 != 0 {
            self.segments.pop_back();
        }
    }

    fn hit_tail(&self, x: u16, y: u16) -> bool {
        for cur in &self.segments {
            if x == cur.pos.x && y == cur.pos.y {
                return true;
            }
        }
        false
    }

    fn hit(&self, food: &Food) -> bool {
        self.segments.front().unwrap().pos == food.pos
    }

}

fn print(snake: &Snake, food: &Food, score: u32) {
    let mut stdout = stdout();
    playfield(&mut stdout, score);
    for cur in &snake.segments {
        stdout
            .queue(cursor::MoveTo(cur.pos.x, cur.pos.y))
            .unwrap()
            .queue(style::PrintStyledContent("*".white()))
            .unwrap();
    }
    stdout
        .queue(cursor::MoveTo(food.pos.x, food.pos.y))
        .unwrap()
        .queue(style::PrintStyledContent("@".blue()))
        .unwrap();
    let head = snake.segments.front().unwrap(); // there's always a head
    let s = format!("x: {}, y: {}", head.pos.x, head.pos.y);
    stdout
        .queue(cursor::MoveTo(COLS + 5, 5))
        .unwrap()
        .queue(style::PrintStyledContent(s.white()))
        .unwrap();
    stdout.flush().ok();
}

fn playfield(stdout: &mut std::io::Stdout, score: u32) {
    stdout
        .execute(terminal::Clear(terminal::ClearType::All))
        .unwrap();
    stdout
        .queue(cursor::MoveTo(0, 0))
        .unwrap()
        .queue(style::PrintStyledContent("*".repeat(COLS as usize).green()))
        .unwrap()
        .queue(cursor::MoveTo(0, ROWS))
        .unwrap()
        .queue(style::PrintStyledContent("*".repeat(COLS as usize).green()))
        .unwrap();
    for r in 0..ROWS {
        stdout
            .queue(cursor::MoveTo(0, r))
            .unwrap()
            .queue(style::PrintStyledContent("*".green()))
            .unwrap()
            .queue(cursor::MoveTo(COLS, r))
            .unwrap()
            .queue(style::PrintStyledContent("*".green()))
            .unwrap();
        let s = format!("Score: {}", score);
        stdout
            .queue(cursor::MoveTo(COLS + 5, 2))
            .unwrap()
            .queue(style::PrintStyledContent(s.white()))
            .unwrap();
    }
}

fn game_over() {
    let mut stdout = stdout();
    stdout
        .queue(cursor::MoveTo(COLS / 2, ROWS / 2))
        .unwrap()
        .queue(style::PrintStyledContent("GAME OVER".red()))
        .unwrap();
    cleanup();
}

fn read_key(snake: &mut Snake) {
    if poll(Duration::from_millis(10)).unwrap_or(false) {
        if let Ok(Event::Key(event)) = read() {
            if event.kind == crossterm::event::KeyEventKind::Press {
                match event.code {
                    KeyCode::Left => {
                        snake.dir = Direction::Left;
                    }
                    KeyCode::Down => {
                        snake.dir = Direction::Down;
                    }
                    KeyCode::Right => {
                        snake.dir = Direction::Right;
                    }
                    KeyCode::Up => {
                        snake.dir = Direction::Up;
                    }
                    KeyCode::Char(c) => {
                        if c == 'q' {
                            cleanup();
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}

fn cleanup() {
    let mut stdout = stdout();
    stdout.queue(cursor::MoveTo(0, ROWS + 2)).ok();
    disable_raw_mode().ok();
    execute!(stdout, Show).ok();
    exit(0);
}

fn main() {
    enable_raw_mode().expect("failed to set raw mode");
    execute!(stdout(), Hide).ok();
    let mut snake = Snake::new();
    let mut tick = 0u64;
    let mut food = Food {
        pos: Location::random(),
    };
    let mut score = 0;
    loop {
        read_key(&mut snake);
        snake.update_snake(tick);
        print(&snake, &food, score);
        if snake.hit(&food) {
            score += 100;
            food = Food {
                pos: Location::random(),
            }
        }
        std::thread::sleep(Duration::from_millis(DELAY));
        tick = tick.wrapping_add(1);
    }
}
