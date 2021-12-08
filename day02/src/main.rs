use std::error::Error as StdError;
use std::str::FromStr;

type Error = Box<dyn StdError>;
type Result<T> = std::result::Result<T, Error>;

type Point = (i32, i32);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Aimed {
  x: i32,
  y: i32,
  aim: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cmd {
  Fwd(i32),
  Down(i32),
  Up(i32),
}

impl FromStr for Cmd {
  type Err = Error;
  fn from_str(s: &str) -> Result<Self> {
    let mut iter = s.split_whitespace();
    let command = iter.next().ok_or("No command")?;
    let value = iter.next().ok_or(Error::from("No parameter")).and_then(|w| w.parse::<i32>().map_err(Error::from))?;
    match command {
      "forward" => Ok(Cmd::Fwd(value)),
      "down" => Ok(Cmd::Down(value)),
      "up" => Ok(Cmd::Up(value)),
      _ => Err("unknown command".into()),
    }
  }
}

fn step(point: Point, cmd: Cmd) -> Point {
  let (x, y) = point;
  match cmd {
    Cmd::Fwd(d) => (x + d, y),
    Cmd::Down(d) => (x, y + d),
    Cmd::Up(d) => (x, y - d),
  }
}

fn step_b(p: Aimed, cmd: Cmd) -> Aimed {
  let Aimed {x, y, aim} = p;
  match cmd {
    Cmd::Fwd(d) => Aimed {x: x + d, y: y + d * aim, aim},
    Cmd::Down(d) => Aimed {x, y, aim: aim + d},
    Cmd::Up(d) => Aimed {x, y, aim: aim - d},
  }
}

fn run_commands(cmds: &[Cmd]) -> Point {
  cmds.iter().copied().fold((0, 0), step)
}

fn run_commands_b(cmds: &[Cmd]) -> Aimed {
  cmds.iter().copied().fold(Aimed {x: 0, y: 0, aim: 0}, step_b)
}

fn part_a(cmds: &[Cmd]) -> i32 {
  let (x_f, y_f) = run_commands(cmds);
  x_f * y_f
}

fn part_b(cmds: &[Cmd]) -> i32 {
  let Aimed {x, y, aim: _aim} = run_commands_b(cmds);
  x * y
}

const PUZZLE: &'static str = include_str!("input02");
const SAMPLE: &'static str = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

fn main() -> Result<()> {
  let parsed: Result<Vec<Cmd>> = PUZZLE.lines().map(|l| l.parse()).collect();
  let parsed = parsed?;
  let soln_a = part_a(&parsed);
  println!("Part a: {}", soln_a);
  let soln_b = part_b(&parsed);
  println!("Part b: {}", soln_b);
  Ok(())
}
