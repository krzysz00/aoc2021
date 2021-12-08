use text_io::scan;

use std::collections::HashSet;
use std::cmp::{min, max};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Cat {
  Horiz,
  Vert,
  Diag,
}


// Invariant: (x1, y1) < (x2, y2)
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct Line {
  pub x1: u32,
  pub y1: u32,
  pub x2: u32,
  pub y2: u32,
  pub category: Cat,
}

impl Line {
  pub fn new(x1: u32, y1: u32, x2: u32, y2: u32) -> Self {
    let (x1, y1, x2, y2) = if (x1, y1) > (x2, y2) { (x2, y2, x1, y1) } else { (x1, y1, x2, y2) };
    let category = if y1 == y2 { Cat::Horiz }
      else if x1 == x2 { Cat::Vert }
      else { Cat::Diag };
    Self {x1, y1, x2, y2, category }
  }

  pub fn parse(line: &str) -> Self {
    let x1: u32;
    let y1: u32;
    let x2: u32;
    let y2: u32;
    scan!(line.bytes() => "{},{} -> {},{}", x1, y1, x2, y2);
    Self::new(x1, y1, x2, y2)
  }
}

fn intersect(a: Line, b: Line, points: &mut HashSet<(u32, u32)>) {
  match (a.category, b.category) {
    // Part a: ignore diagonal lines
    // (Cat::Diag, _) => (),
    // Part b: Handle diagonal lines
    (Cat::Diag, Cat::Horiz) => {
      let y = b.y1 as i32;
      let y_start = a.y1 as i32;
      let y_end = a.y2 as i32;
      let x_start = a.x1 as i32;
      let x_end = a.x2 as i32;
      let slope = if y_end < y_start { -1 } else { 1 };
      let dy = y - y_start;
      let x_test = slope * dy + x_start;
      if x_test >= x_start && x_test <= x_end && x_test >= b.x1 as i32 && x_test <= b.x2 as i32 {
        points.insert((x_test as u32, b.y1));
      }
    },
    (Cat::Diag, Cat::Vert) => {
      let x = b.x1 as i32;
      let x_start = a.x1 as i32;
      let y_start = a.y1 as i32;
      let y_end = a.y2 as i32;

      let dx = x - x_start;
      let slope = if y_end < y_start { -1 } else { 1 };
      let y_test = slope * dx + y_start;
      let y_min = min(y_start, y_end);
      let y_max = max(y_start, y_end);
      if y_test >= y_min && y_test <= y_max && y_test >= b.y1 as i32 && y_test <= b.y2 as i32 {
        points.insert((b.x1, y_test as u32));
      }
    },
    (Cat::Diag, Cat:: Diag) => {
      // In general, we want to solve
      // m1x + b1 = m2x = + b2
      // or x = (m2 - b1)/(m2 - m1)
      // Converiently, all the m are +- 1
      let m1 = if a.y1 > a.y2 { -1 } else { 1 };
      let m2 = if b.y1 > b.y2 { -1 } else { 1 };
      let b1 = a.y1 as i32 - m1 * a.x1 as i32;
      let b2 = b.y1 as i32 - m2 * b.x1 as i32;

      if m1 == m2 {
        if b1 != b2 {
          return;
        }
        let x_min = max(a.x1, b.x1);
        let x_max = min(a.x2, b.x2);
        // Same slope: mutual overlap or parallel
        for x in x_min ..= x_max {
          let y = (m1 * (x as i32) + b1) as u32;
          points.insert((x, y));
        }
      }
      else {
        let b_diff = b2 - b1;
        if b_diff.abs() % 2 != 0 {
          return;
        }
        let new_x  = b_diff / (m1 - m2);
        let new_y = m1 * new_x + b1;
        let new_x = new_x as u32;
        let new_y = new_y as u32;
        if new_x >= a.x1 && new_x <= a.x2 && new_x >= b.x1 && new_x <= b.x2 {
          points.insert((new_x, new_y));
        }
      }
    }
    (_, Cat::Diag) => intersect(b, a, points),
    (Cat::Vert, Cat::Horiz) => intersect(b, a, points),
    (Cat::Horiz, Cat::Vert) => {
      if a.x1 <= b.x1 && b.x1 <= a.x2 && b.y1 <= a.y1 && a.y1 <= b.y2 {
        points.insert((b.x1, a.y1));
      }
    },
    (Cat::Horiz, Cat::Horiz) => {
      if a.y1 == b.y1 {
        let y = a.y1;
        let i_min = max(a.x1, b.x1);
        let i_max = min(a.x2, b.x2);
        for i in i_min ..= i_max {
          points.insert((i, y));
        }
      }
    },
    (Cat::Vert, Cat::Vert) => {
      if a.x1 == b.x1 {
        let x = a.x1;
        let i_min = max(a.y1, b.y1);
        let i_max = min(a.y2, b.y2);
        for i in i_min ..= i_max {
          points.insert((x, i));
        }
      }
    },
  }
}

fn solve(lines: &[Line]) -> usize {
  let mut points = HashSet::<(u32, u32)>::new();
  for (i, l1) in lines.iter().enumerate() {
    for l2 in &lines[(i+1)..] {
      intersect(*l1, *l2, &mut points);
    }
  }
  points.len()
}

fn main() {
  let parsed: Vec<Line> = PUZZLE.lines().map(Line::parse).collect();
  let soln = solve(&parsed);
  // Part a vs b are comment things out
  println!("Solution: {}", soln);
}


const PUZZLE: &'static str = include_str!("input05");
const SAMPLE: &'static str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";
