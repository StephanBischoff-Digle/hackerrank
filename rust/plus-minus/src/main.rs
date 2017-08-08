#[macro_use]
extern crate log;
extern crate env_logger;

fn read_input() -> Vec<String> {
    debug!("reading input");
    use std::io::BufRead;
    let input = std::io::stdin();
    let lines = input.lock().lines();
    let ret: Vec<String> = lines.filter_map(|s| s.ok()).collect();

    debug!("{} lines read", ret.len());
    ret
}



fn main() {
    env_logger::init().unwrap();
    let input = read_input();

    let n = match input[0].parse::<usize>().ok() {
        Some(x) => x,
        None => 0,
    };

    let sums = input[1]
        .split_whitespace()
        .take(n)
        .map(|x| match x.parse::<i32>().ok() {
            Some(x) => x,
            None => 0,
        })
        .fold(Scores::new(), |acc, x| {
            acc + match x {
                a if a > 0 => Scores::new_init(1, 0, 0),
                a if a < 0 => Scores::new_init(0, 1, 0),
                a if a == 0 => Scores::new_init(0, 0, 1),
                _ => Scores::new(),
            }
        });

    debug!(
        "sums: P:{}, N:{}, Z:{}",
        sums.positive,
        sums.negative,
        sums.zero
    );

    let total = sums.positive + sums.negative + sums.zero;
    println!("{}", sums.positive as f64 / total as f64);
    println!("{}", sums.negative as f64 / total as f64);
    println!("{}", sums.zero as f64 / total as f64);
}

#[derive(Debug)]
struct Scores {
    positive: u16,
    negative: u16,
    zero: u16,
}

impl Scores {
    fn new() -> Scores {
        debug!("creating new Scores.");
        Scores {
            positive: 0,
            negative: 0,
            zero: 0,
        }
    }

    fn new_init(pos: u16, neg: u16, zero: u16) -> Scores {
        debug!("creating new Scores with P:{}, N:{}, Z:{}.", pos, neg, zero);
        Scores {
            positive: pos,
            negative: neg,
            zero: zero,
        }
    }
}

use std::ops::Add;
impl Add for Scores {
    type Output = Scores;

    fn add(self, other: Scores) -> Scores {
        Scores {
            positive: self.positive + other.positive,
            negative: self.negative + other.negative,
            zero: self.zero + other.zero,
        }
    }
}
