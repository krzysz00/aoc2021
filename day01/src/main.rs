const PUZZLE: &'static str = include_str!("input01");

fn parse(input: &str) -> Vec<u32> {
  input.lines().map(|l| l.parse::<u32>().expect("input has integers")).collect()
}

fn part_a(input: &[u32]) -> u32 {
  input.windows(2).filter(|w| w[1] > w[0]).count() as u32
}

fn part_b(input: &[u32]) -> u32 {
  let conved: Vec<u32> = input.windows(3).map(|w| w.iter().copied().sum::<u32>()).collect();
  part_a(&conved)
}

fn main() {
  let data = parse(PUZZLE);
  let soln_a = part_a(&data);
  println!("Part a: {}", soln_a);
  let soln_b = part_b(&data);
  println!("Part b: {}", soln_b);
}
