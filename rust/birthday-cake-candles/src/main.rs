
fn read_input() -> Vec<String> {
    use std::io::BufRead;
    let input = std::io::stdin();
    let lines = input.lock().lines();
    let ret: Vec<String> = lines.filter_map(|s| s.ok()).collect();
    ret
}

fn main() {
    let input = read_input();

    let n = match input[0].parse::<usize>().ok() {
        Some(x) => x,
        None => 0,
    };

    let values: Vec<u32> = input[1]
        .split_whitespace()
        .map(|x| match x.parse::<u32>().ok() {
            Some(x) => x,
            None => 0,
        })
        .take(n)
        .collect();

    let max: &u32 = values.iter().max().unwrap();
    let n_max = values.iter().filter(|&x| *x == *max).count();

    println!("{}", n_max);
}
