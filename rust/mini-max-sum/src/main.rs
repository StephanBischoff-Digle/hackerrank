
fn read_input() -> Vec<String> {
    use std::io::BufRead;
    let input = std::io::stdin();
    let lines = input.lock().lines();
    let ret: Vec<String> = lines.filter_map(|s| s.ok()).collect();
    ret
}

fn main() {
    let input = read_input();

    let mut values: Vec<u64> = input[0]
            .split_whitespace()
            .map(|x| match x.parse::<u64>().ok() {
                Some(x) => x,
                None => 0,
            }).collect();

    values.sort();
    let min: u64 = values.iter().take(4).sum();
    let max: u64 = values.iter().skip(1).sum();

    println!("{} {}", min, max);
}
