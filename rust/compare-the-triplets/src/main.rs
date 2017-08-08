fn main() {
    use std::io::BufRead;

    let input = std::io::stdin();
    let lines: Vec<String> = input.lock().lines()
        .filter_map(|s| s.ok())
        .collect();
    let alice: Vec<u16> = lines[0]
        .split_whitespace()
        .map(|x| match x.parse::<u16>().ok() {
            Some(x) => x,
            None => 0,
        }) .collect();

    let bob: Vec<u16> = lines[1]
        .split_whitespace()
        .map(|x| match x.parse::<u16>().ok() {
            Some(x) => x,
            None => 0,
        }) .collect();

    let mut score_alice = 0u16;
    let mut score_bob = 0u16;
    for i in 0..bob.len() {
        if alice[i] > bob[i] {
            score_alice += 1;
        } else if alice[i] < bob[i] {
            score_bob += 1;
        }
    }

    println!("{} {}", score_alice, score_bob);
}
