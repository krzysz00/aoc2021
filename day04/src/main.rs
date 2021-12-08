use std::error::Error as StdError;
type Error = Box<dyn StdError>;
type Result<T> = std::result::Result<T, Error>;

const N: usize = 5;
const SIZE: usize = N * N;

fn parse(input: &str) -> Result<(Vec<u8>, Vec<Vec<u8>>)> {
  let mut lines = input.lines();
  let first_line = lines.next().ok_or(Error::from("missing draw numbers"))?;
  let draws: Result<Vec<u8>> = first_line.split(',').map(|n| n.parse().map_err(Error::from)).collect();
  let draws = draws?;
  let split = lines.next();
  if Some("") != split {
    return Err("Missing blank line between numbers and boards".into());
  }
  let mut boards: Vec<Vec<u8>> = Vec::new();
  let mut board = Vec::<u8>::new();
  while let Some(line) = lines.next() {
    if line == "" {
      if board.len() == 0 {
        continue;
      }
      boards.push(board);
      board = Vec::new();
    }
    else {
      for n in line.split_ascii_whitespace() {
        let n = n.parse::<u8>()?;
        board.push(n);
      }
    }
  }
  if board.len() != 0 {
    boards.push(board);
  }
  if !boards.iter().all(|b| b.len() == SIZE) {
    return Err("Boards not of correct size were parsed".into());
  }
  Ok((draws, boards))
}


// Mark off the square labelled `n` on `board`, if any, by setting the corresponding bit in the `marks` array. Return `true` if marking was performed
fn mark(board: &[u8], marks: &mut [bool], n: u8) -> bool {
  assert!(board.len() == marks.len());
  for (i, m) in board.iter().zip(marks.iter_mut()) {
    if *i == n {
      *m = true;
      return true;
    }
  }
  return false;
}

fn has_bingo(marks: &[bool]) -> bool {
  assert!(marks.len() == N * N);
  let row_bingo = marks.chunks_exact(N).any(|c| c.iter().copied().all(|x| x));
  let column_bingo = (0..N).any(|j| (0..N).all(|i| marks[j + N * i]));
  row_bingo || column_bingo
}

fn score_unmarked(board: &[u8], marks: &[bool]) -> u64 {
  board.iter().copied().zip(marks.iter().copied())
  .filter_map(|(n, m)| if m { None } else { Some (n as u64) })
  .sum()
}

fn part_a(draws: &[u8], boards: &[Vec<u8>]) -> u64 {
  let mut marks: Vec<Vec<bool>> = (0..boards.len()).map(|_| vec![false; SIZE]).collect();
  for drawn in draws.iter().copied() {
    for (board, mut marks) in boards.iter().zip(marks.iter_mut()) {
      if mark(&board, &mut marks, drawn) {
        if has_bingo(&marks) {
          return score_unmarked(&board, &marks) * (drawn as u64);
        }
      }
    }
  }
  panic!("No one got a bingo");
}

fn part_b(draws: &[u8], boards: &[Vec<u8>]) -> u64 {
  let n_boards = boards.len();
  let mut marks: Vec<Vec<bool>> = (0..n_boards).map(|_| vec![false; SIZE]).collect();
  let mut already_won = vec![false; n_boards];
  let mut bingo_count = 0;
  for d in draws.iter().copied() {    
    for (already_won, (board, mut marks)) in already_won.iter_mut().zip(boards.iter().zip(marks.iter_mut())).filter(|t| !*t.0) {
      if mark(&board, &mut marks, d) {
        if has_bingo(&marks) {
          *already_won = true;
          bingo_count += 1;
          if bingo_count == n_boards {
            return score_unmarked(&board, &marks) * (d as u64);
          }
        }
      }
    }
  }
  panic!("No last bingo somehow");
}

fn main() -> Result<()> {
  let (draws, boards) = parse(PUZZLE)?;
  let soln_a = part_a(&draws, &boards);
  println!("Part a: {}", soln_a);
  let soln_b = part_b(&draws, &boards);
  println!("Part b: {}", soln_b);
  Ok(())
}

const PUZZLE: &'static str = include_str!("input04");

const SAMPLE: &'static str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11 0
 8  2 23  4 24
21 9 14 16 7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24 4
14 21 16 12 6

14 21 17 24 4
10 16 15  9 19
18 8 23 26 20
22 11 13  6  5
 2  0 12  3  7";