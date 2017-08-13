use std::collections::HashMap;

fn read_input() -> Vec<String> {
    use std::io::BufRead;
    let input = std::io::stdin();
    let lines = input.lock().lines();
    let ret: Vec<String> = lines.filter_map(|s| s.ok()).collect();

    ret
}


fn get_weight_map() -> HashMap<char, u32> {
    let mut weights = HashMap::new();

    weights.insert('a', 1);
    weights.insert('b', 2);
    weights.insert('c', 3);
    weights.insert('d', 4);
    weights.insert('e', 5);
    weights.insert('f', 6);
    weights.insert('g', 7);
    weights.insert('h', 8);
    weights.insert('i', 9);
    weights.insert('j', 10);
    weights.insert('k', 11);
    weights.insert('l', 12);
    weights.insert('m', 13);
    weights.insert('n', 14);
    weights.insert('o', 15);
    weights.insert('p', 16);
    weights.insert('q', 17);
    weights.insert('r', 18);
    weights.insert('s', 19);
    weights.insert('t', 20);
    weights.insert('u', 21);
    weights.insert('v', 21);
    weights.insert('w', 22);
    weights.insert('x', 23);
    weights.insert('y', 25);
    weights.insert('z', 26);

    weights
}

fn main() {
    let input = read_input();

    let string = &input[0].trim().to_lowercase();

    let n = input[1].parse::<usize>().unwrap_or_else(|_| 0);
    let vals: Vec<u32> = input
        .iter()
        .skip(2)
        .map(|x| x.parse::<u32>().unwrap_or_else(|_| 0))
        .collect();

    let map = get_weight_map();
    let mut substrings = HashMap::<String, u32>::new();

    let mut prev_string = '0';
    let mut occ_counter = 1;
    for c in string.chars() {
        if c != prev_string {
            let val = match map.get(&c) {
                Some(x) => *x,
                None => 0,
            };
            let mut in_string = String::new();
            in_string.push(c);
            substrings.insert(in_string.clone(), val);
            prev_string = c;
            occ_counter = 1;
        } else {
            let mut in_string = String::new();
            occ_counter += 1;
            let val = match map.get(&c) {
                Some(x) => *x,
                None => 0,
            } * occ_counter;

            for _ in 0..occ_counter {
                in_string.push(c);
            }
            substrings.insert(in_string.clone(), val);
        }
    }

    for (i,&v) in vals.iter().enumerate() {
        if v != 0 && i < n {
            if substrings.values().any(|x| *x == v) {
                println!("Yes");
            } else {
                println!("No");
            }
        }
    }
}
