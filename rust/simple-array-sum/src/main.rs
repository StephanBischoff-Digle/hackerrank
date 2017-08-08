fn main() {
    use std::io::BufRead;

    let input = std::io::stdin();
    let line_iter = input.lock().lines()
        .filter_map(|s| s.ok());

    let lines: Vec<String> = line_iter.collect();
    let n = match lines[0].parse::<usize>().ok() {
        Some(x) => x,
        None => 0,
    };

    let values: Vec<i32> = lines[1]
        .split_whitespace()
        .map(|x| match x.parse::<i32>().ok() {
            Some(x) => x,
            None => 0,
        })
        .collect();

    println!("{}", values.iter().take(n).sum::<i32>());
}
