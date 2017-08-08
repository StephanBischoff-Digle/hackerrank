fn read_input() -> Vec<String>
{
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

    let mut diags = (0i32, 0i32);
    for i in 0..n {
        let values: Vec<i32> = input[i+1]
            .split_whitespace()
            .map(|x| match x.parse::<i32>().ok() {
                Some(x) => x,
                None => 0,
            }).collect();

        diags.0 += values[i];
        diags.1 += values[n - 1 - i];
    }


    println!("{}", (diags.0 - diags.1).abs());
}
