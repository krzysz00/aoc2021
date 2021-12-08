use std::error::Error as StdError;

type Error = Box<dyn StdError>;
type Result<T> = std::result::Result<T, Error>;

const PUZZLE: &'static str = include_str!("input03");
const SAMPLE: &'static str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010
";

const PUZZLE_N: u16 = 12;
const SAMPLE_N: u16 = 5;

fn parse(line: &str) -> Result<u16> {
  u16::from_str_radix(line, 2).map_err(Error::from)
}

fn bit_count(input: &[u16], n: u16) -> Vec<(u16, u16)> {
  let mut ret = vec![(0, 0); n as usize];
  for k in input.iter().copied() {
    for i in 0..n {
      let bit = (k >> i) & 1;
      match bit {
        0 => ret[i as usize].0 += 1,
        1 => ret[i as usize].1 += 1,
        _ => ()
      };
    }
  }
  return ret;
}

fn part_a(input: &[u16], n: u16) -> u32 {
  let counts = bit_count(input, n);
  let mut gamma: u16 = 0;
  let mut epsilon: u16 = 0;
  for (i, (zero, one)) in counts.into_iter().enumerate() {
    let i = i as u16;
    if zero > one {
      epsilon |= 1 << i;
    }
    else if one > zero {
      gamma |= 1 << i;
    }
  }
  (gamma as u32) * (epsilon as u32)
}

fn most_common_bit(input: &[u16], n: u16) -> bool {
  let mut count_0 = 0;
  let mut count_1 = 0;
  for s in input.iter().copied() {
    if ((s >> n) & 1) != 0 {
      count_1 += 1;
    } else {
      count_0 += 1;
    }
  }
  return count_1 >= count_0;
}

fn parity_filter(numbers: &mut Vec<u16>, n: u16, invert_common: bool) {
  let filter = most_common_bit(&numbers, n);
  let filter = (filter ^ invert_common) as u16;
  let mut kept_idx = 0;
  let orig_size = numbers.len();
  for i in 0..orig_size {
    if ((numbers[i] >> n) & 1) == filter {
      numbers[kept_idx] = numbers[i];
      kept_idx += 1;
    }
  }
  println!("Filter = {}, (xor = {:?}) i = {}, old len = {}, new len = {}", filter, invert_common, n, orig_size, kept_idx);
  numbers.truncate(kept_idx);
}

fn part_b(mut for_o2: Vec<u16>, n: u16) -> Result<u32> {
  let mut for_co2 = for_o2.clone();
  for i in (0..n).rev() {
    if for_o2.len() > 1 {
      parity_filter(&mut for_o2, i, false);
    }
    if for_co2.len() > 1 {
      parity_filter(&mut for_co2, i, true);
    }
  }
  if for_o2.len() != 1 {
    return Err("Couldn't filter out an O2 reading".into());
  }
  if for_co2.len() != 1 {
    return Err("Couldn't filter out a CO2 reading".into());
  }
  Ok((for_o2[0] as u32) * (for_co2[0] as u32))
}

fn main() -> Result<()> {
  let parsed: Result<Vec<u16>> = PUZZLE.lines().map(parse).collect();
  let parsed = parsed?;
  let soln_a = part_a(&parsed, PUZZLE_N);
  println!("Part a: {}", soln_a);
  let soln_b = part_b(parsed, PUZZLE_N)?;
  println!("Part b: {}", soln_b);
  Ok(())
}
