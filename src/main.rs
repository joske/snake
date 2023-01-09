use crossterm::{
    cursor,
    event::{poll, read, Event, KeyCode, KeyEvent},
    style::{self, Stylize},
    terminal::{self, enable_raw_mode},
    ExecutableCommand, QueueableCommand, Result,
};
use std::{
    fmt,
    io::{stdout, Write},
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
    head: Box<Segment>,
}

impl Snake {
    pub fn new() -> Self {
        Snake {
            head: Box::new(Segment {
                next: None,
                pos: Location { x: 1, y: 0 },
                dir: Direction::Right,
            }),
        }
    }
}

#[derive(Debug)]
struct Segment {
    next: Option<Box<Segment>>,
    pos: Location,
    dir: Direction,
}

fn main() {
    enable_raw_mode().expect("failed to set raw mode");
    let mut snake = Snake::new();
    snake.head.next = Some(Box::new(Segment {
        next: None,
        pos: Location { x: 0, y: 0 },
        dir: Direction::Right,
    }));
    let mut tick = 0u64;
    loop {
        print_snake(&snake);
        read_key(&mut snake);
        if tick % 5 == 0 {
            add_segment(&mut snake);
        }
        update_snake(&mut snake);
        std::thread::sleep(Duration::from_millis(500));
        tick += 1;
    }
}

fn add_segment(snake: &mut Snake) {
    let p = &snake.head;
    while let Some(t) = p.next {
        p = &t;
    }
    let new_segment = Segment {
        pos: p.pos.clone(),
        dir: p.dir.clone(),
        next: None,
    };
    p.next = Some(Box::new(new_segment));
}

fn read_key(snake: &mut Snake) {
    if poll(Duration::from_millis(100)).unwrap_or(false) {
        println!("got event");
        match read() {
            Ok(Event::Key(event)) => {
                println!("got event {:?}", event);
                if event.kind == crossterm::event::KeyEventKind::Press {
                    match event.code {
                        KeyCode::Left => {
                            snake.head.dir = Direction::Left;
                        }
                        KeyCode::Down => {
                            snake.head.dir = Direction::Down;
                        }
                        KeyCode::Right => {
                            snake.head.dir = Direction::Right;
                        }
                        KeyCode::Up => {
                            snake.head.dir = Direction::Up;
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
}

fn print_snake(snake: &Snake) {
    let mut stdout = stdout();
    stdout.execute(terminal::Clear(terminal::ClearType::All));

    let mut cur = &snake.head;
    stdout
        .queue(cursor::MoveTo(cur.pos.x, cur.pos.y))
        .unwrap()
        .queue(style::PrintStyledContent("*".white()));
    while let Some(next) = &cur.next {
        stdout
            .queue(cursor::MoveTo(next.pos.x, next.pos.y))
            .unwrap()
            .queue(style::PrintStyledContent("=".white()));
        cur = next;
    }

    let s = format!("snake: {:?}", snake);

    stdout
        .queue(cursor::MoveTo(0, 2))
        .unwrap()
        .queue(style::Print(s));

    stdout.flush();
}

fn update_snake(snake: &mut Snake) {
    let mut cur = &mut snake.head;
    update_segment(&mut cur);
    while let Some(next) = &mut cur.next {
        update_segment(next);
        if cur.dir != next.dir {
            next.dir = cur.dir.clone();
        }
        cur = next;
    }
}

fn update_segment(cur: &mut Segment) {
    match cur.dir {
        Direction::Up => cur.pos.y = cur.pos.y - 1,
        Direction::Down => cur.pos.y = cur.pos.y + 1,
        Direction::Left => cur.pos.x = cur.pos.x - 1,
        Direction::Right => cur.pos.x = cur.pos.x + 1,
    }
}
