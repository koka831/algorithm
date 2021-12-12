/// https://adventofcode.com/2021/day/2
/// Day 2: Dive!
use std::fs::File;
use std::io::{BufRead, BufReader};

enum MoveKind {
    Forward,
    Up,
    Down,
}

#[derive(Default)]
struct State {
    depth: usize,
    position: usize,
}

struct Command {
    move_kind: MoveKind,
    unit: usize,
}

impl Command {
    fn parse(input: &str) -> Self {
        let (move_kind, unit) = if let Some(i) = input.strip_prefix("forward ") {
            (MoveKind::Forward, i)
        } else if let Some(i) = input.strip_prefix("up ") {
            (MoveKind::Up, i)
        } else if let Some(i) = input.strip_prefix("down ") {
            (MoveKind::Down, i)
        } else {
            unreachable!()
        };

        Command {
            move_kind,
            unit: unit.parse().unwrap(),
        }
    }

    fn execute(&self, state: State) -> State {
        match self.move_kind {
            MoveKind::Forward => State {
                position: state.position + self.unit,
                ..state
            },
            MoveKind::Up => State {
                depth: state.depth - self.unit,
                ..state
            },
            MoveKind::Down => State {
                depth: state.depth + self.unit,
                ..state
            },
        }
    }
}

fn main() {
    let file = File::open("./input/day2_1.txt").unwrap();
    let lines = BufReader::new(&file).lines();
    let mut state = State::default();
    for line in lines {
        state = Command::parse(&line.unwrap()).execute(state);
    }

    println!("{}", state.depth * state.position);
}
