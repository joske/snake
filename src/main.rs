use crossterm::{
    cursor::{self, Hide},
    event::{poll, read, Event, KeyCode},
    execute,
    style::{self, Stylize},
    terminal::{self, disable_raw_mode, enable_raw_mode},
    ExecutableCommand, QueueableCommand,
};
use std::{
    collections::LinkedList,
    io::{stdout, Write},
    process::exit,
    time::Duration,
};

const COLS: u16 = 20;
const ROWS: u16 = 20;

#[derive(Debug, Clone)]
struct Location {
    x: u16,
    y: u16,
}

#[derive(Debug, Clone, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
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
                    pos: Location { x: 2, y: 0 },
                },
                Segment {
                    pos: Location { x: 1, y: 0 },
                },
                Segment {
                    pos: Location { x: 0, y: 0 },
                },
            ]),
            dir: Direction::Right,
        }
    }

    fn add_segment(&mut self) {
        let last = self.segments.back().unwrap(); // always at least 1 segment
        let (x, y) = match self.dir {
            Direction::Left => (last.pos.x + 1, last.pos.y),
            Direction::Right => (last.pos.x - 1, last.pos.y),
            Direction::Up => (last.pos.x, last.pos.y + 1),
            Direction::Down => (last.pos.x, last.pos.y - 1),
        };
        let s = Segment {
            pos: Location { x, y },
        };
        self.segments.push_back(s);
    }

    fn print_snake(&self) {
        let mut stdout = stdout();
        stdout
            .execute(terminal::Clear(terminal::ClearType::All))
            .ok();

        for cur in self.segments.iter() {
            stdout
                .queue(cursor::MoveTo(cur.pos.x, cur.pos.y))
                .unwrap()
                .queue(style::PrintStyledContent("*".white()))
                .ok();
        }
        stdout.flush().ok();
    }

    fn update_snake(&mut self, tick: u64) {
        let head = self.segments.front().unwrap(); // there's always a head

        let (x, y) = match self.dir {
            Direction::Up => (head.pos.x, head.pos.y - 1),
            Direction::Down => (head.pos.x, head.pos.y + 1),
            Direction::Left => (head.pos.x - 1, head.pos.y),
            Direction::Right => (head.pos.x + 1, head.pos.y),
        };
        let new_head = Segment {
            pos: Location { x, y },
        };
        self.segments.push_front(new_head);
        if tick % 5 != 0 {
            self.segments.pop_back();
        }
    }
}

#[derive(Debug)]
struct Segment {
    pos: Location,
}

fn main() {
    enable_raw_mode().expect("failed to set raw mode");
    execute!(stdout(), Hide).ok();
    let mut snake = Snake::new();
    let mut tick = 0u64;
    loop {
        snake.print_snake();
        read_key(&mut snake);
        snake.update_snake(tick);
        std::thread::sleep(Duration::from_millis(500));
        tick = tick.wrapping_add(1);
    }
}

fn read_key(snake: &mut Snake) {
    if poll(Duration::from_millis(100)).unwrap_or(false) {
        match read() {
            Ok(Event::Key(event)) => {
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
                                disable_raw_mode().ok();
                                exit(0);
                            }
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
}
