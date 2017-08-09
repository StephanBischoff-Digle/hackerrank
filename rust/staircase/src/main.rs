
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

    let space = " ";
    let stair = "#";
    for row in 1..n+1 {
        let mut tmp_str = String::new();
        tmp_str.push_str(&(space.repeat(n-row)));
        tmp_str.push_str(&(stair.repeat(row)));
        println!("{}", tmp_str);
    }
}
