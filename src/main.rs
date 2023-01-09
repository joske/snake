use crossterm::{
    cursor::{self, Hide, Show},
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
                .unwrap();
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
        if x == 0 || x > COLS || y == 0 || y > ROWS {
            // kapoet
            game_over();
        }
        let new_head = Segment {
            pos: Location { x, y },
        };
        self.segments.push_front(new_head);
        if tick % 5 != 0 {
            self.segments.pop_back();
        }
    }
}

fn game_over() {
    let mut stdout = stdout();
    stdout
        .queue(cursor::MoveTo(COLS / 2, ROWS / 2))
        .unwrap()
        .queue(style::PrintStyledContent("GAME OVER".red()))
        .unwrap()
        .queue(cursor::MoveTo(0, ROWS))
        .ok();
    disable_raw_mode().ok();
    execute!(stdout, Show).ok();
    exit(0);
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
                            if snake.dir == Direction::Right {
                                game_over();
                            }
                            snake.dir = Direction::Left;
                        }
                        KeyCode::Down => {
                            if snake.dir == Direction::Up {
                                game_over();
                            }
                            snake.dir = Direction::Down;
                        }
                        KeyCode::Right => {
                            if snake.dir == Direction::Left {
                                game_over();
                            }
                            snake.dir = Direction::Right;
                        }
                        KeyCode::Up => {
                            if snake.dir == Direction::Down {
                                game_over();
                            }
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
