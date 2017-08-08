fn main() {
    use std::io::BufRead;

    let input = std::io::stdin();
    let lines: Vec<String> = input.lock().lines()
        .filter_map(|s| s.ok())
        .collect();

    let n = match lines[0].parse::<usize>().ok() {
        Some(x) => x,
        None => 0,
    };
    let values: Vec<u64> = lines[1]
        .split_whitespace()
        .map(|x| match x.parse::<u64>().ok() {
            Some(x) => x,
            None => 0,
        })
        .collect();

    println!("{}", values.iter().take(n).sum::<u64>());
}
