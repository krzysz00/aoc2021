#[derive(Clone,Copy,PartialEq,Eq)]
enum Tag {
    Paren, Square, Curly, Angle,
}

#[derive(Clone,Copy,PartialEq,Eq)]
enum Token {
    Open(Tag),
    Close(Tag),
}

fn lex(b: u8) -> Option<Token> {
    use Token::*;
    use Tag::*;
    match b {
        b'(' => Some(Open(Paren)),
        b')' => Some(Close(Paren)),
        b'[' => Some(Open(Square)),
        b']' => Some(Close(Square)),
        b'{' => Some(Open(Curly)),
        b'}' => Some(Close(Curly)),
        b'<' => Some(Open(Angle)),
        b'>' => Some(Close(Angle)),
        _ => None,
    }
}

fn parse(input: &str) -> Vec<Vec<Token>> {
    input.lines().map(|l| l.bytes().filter_map(lex).collect()).collect()
}

#[derive(Clone,PartialEq,Eq)]
enum WalkResult {
    Mismatch(Tag),
    Unclosed(Vec<Tag>),
    Other,
}

fn walk(line: &[Token]) -> WalkResult {
    let mut stack = Vec::new();
    for t in line.iter().copied() {
        match t {
            Token::Open(tag) => {
                stack.push(tag);
            },
            Token::Close(tag) => {
                if let Some(matched) = stack.pop() {
                    if tag != matched {
                        return WalkResult::Mismatch(tag);
                    }
                } else {
                    return WalkResult::Other;
                }
            }
        }
    }
    if stack.len() > 0 {
        WalkResult::Unclosed(stack)
    } else {
        WalkResult::Other
    }
}

fn score(tag: Tag) -> u64 {
    use Tag::*;
    match tag {
        Paren => 3,
        Square => 57,
        Curly => 1197,
        Angle => 25137,
    }
}

fn score_b(tag: Tag) -> u64 {
    use Tag::*;
    match tag {
        Paren => 1,
        Square => 2,
        Curly => 3,
        Angle => 4,
    }
}

fn part_a(input: &[Vec<Token>]) -> u64 {
    let mut ret = 0;
    for line in input {
        match walk(line) {
            WalkResult::Mismatch(tag) => ret += score(tag),
            _ => (),
        }
    }
    ret
}

fn part_b(input: &[Vec<Token>]) -> u64 {
    let mut results = Vec::new();
    for line in input {
        match walk(line) {
            WalkResult::Unclosed(tags) => {
                results.push(tags.into_iter().rev().fold(0, |s, t| score_b(t) + 5 * s))
            },
            _ => (),
        }
    }
    results.sort_unstable();
    results[results.len() / 2]
}

fn main() {
    let input_str =
        if std::env::args().any(|x| x == "sample") { SAMPLE } else { PUZZLE };
    let input = parse(input_str);
    let soln_a = part_a(&input);
    println!("Part a: {}", soln_a);
    let soln_b = part_b(&input);
    println!("Part b: {}", soln_b);
}

const PUZZLE: &'static str = include_str!("input10");
const SAMPLE: &'static str =
"[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";
