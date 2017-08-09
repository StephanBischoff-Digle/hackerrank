
fn read_input() -> Vec<String> {
    use std::io::BufRead;
    let input = std::io::stdin();
    let lines = input.lock().lines();
    let ret: Vec<String> = lines.filter_map(|s| s.ok()).collect();
    ret
}

fn main() {
    let input = read_input();

    let split = input[0].split(':').collect::<Vec<&str>>();
    let mut hour = match split[0].parse::<u16>().ok() {
        Some(x) => x,
        None => 0,
    };
    let minute = match split[1].parse::<u16>().ok() {
        Some(x) => x,
        None => 0,
    };

    if split[2].contains("PM") && hour != 12 {
        hour += 12;
    } else if split[2].contains("AM") && hour == 12 {
        hour = 0;
    }
    let second = match split[2]
        .chars()
        .take(2)
        .collect::<String>()
        .parse::<u16>()
        .ok()
    {
        Some(x) => x,
        None => 0,
    };

    println!("{:02}:{:02}:{:02}", hour, minute, second);
}
