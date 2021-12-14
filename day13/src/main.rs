use fxhash::FxHashSet;

use std::error::Error as StdError;
use std::fmt;

type Error = Box<dyn StdError>;
type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
enum Axis {
    X, Y,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    pub fn update(&self, axis: Axis, p: i32) -> Self {
        match axis {
            Axis::X => Self {x: p, y: self.y},
            Axis::Y => Self {x: self.x, y: p},
        }
    }

    pub fn get(&self, axis: Axis) -> i32 {
        match axis {
            Axis::X => self.x,
            Axis::Y => self.y,
        }
    }
}

impl std::str::FromStr for Point {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        match s.split_once(',') {
            Some((x, y)) => {
                let x = x.parse()?;
                let y = y.parse()?;
                Ok(Point::new(x, y))
            },
            None => {
                Err("No comma in point".into())
            }
        }
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Fold {
    pub axis: Axis,
    pub coord: i32,
}

impl Fold {
    pub fn new(axis: Axis, coord: i32) -> Self {
        Self { axis, coord }
    }
}

fn parse_fold(line: &str) -> Result<Fold> {
    let (dir, coord) = line.split_once('=')
        .ok_or_else(|| Error::from("No = in fold"))?;
    let dir = dir.as_bytes()[dir.len() - 1];
    let coord = coord.parse()?;
    match dir {
        b'x' => Ok(Fold::new(Axis::X, coord)),
        b'y' => Ok(Fold::new(Axis::Y, coord)),
        b => Err(format!("Unexpected fold axis {}", b).into()),
    }
}

fn parse(input_str: &str) -> Result<(FxHashSet<Point>, Vec<Fold>)> {
    let (points, folds) = input_str.split_once("\n\n")
        .ok_or_else(|| Error::from("No split point before folds"))?;
    let points: Result<FxHashSet<Point>> =
        points.lines().map(|l| l.trim().parse()).collect();
    let points = points?;
    let folds: Result<Vec<Fold>> =
        folds.lines().map(|l| parse_fold(l.trim())).collect();
    let folds = folds?;
    Ok((points, folds))
}

fn apply_fold(input: &FxHashSet<Point>, fold: Fold) -> FxHashSet<Point> {
    let Fold {axis, coord} = fold;
    // Fold accoss y=N works on the x coordinate and vice versa
    input.iter().copied().map(move |p| {
        if p.get(axis) >= coord {
            let dist = p.get(axis) - coord;
            p.update(axis, coord - dist)
        } else {
            p
        }
    }).collect()
}

fn part_a(points: &FxHashSet<Point>, folds: &[Fold]) -> usize {
    let new_points = apply_fold(points, folds[0]);
    new_points.len()
}

fn main() -> Result<()> {
    let input_str =
        if std::env::args().any(|x| x == "sample") { SAMPLE } else { PUZZLE };
    let (points, folds) = parse(input_str)?;
    let soln_a = part_a(&points, &folds);
    println!("Part a: {}", soln_a);
    Ok(())
}

const PUZZLE: &'static str = include_str!("input13");
const SAMPLE: &'static str =
"6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";
