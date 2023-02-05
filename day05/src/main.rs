use regex::Regex;
use std::collections::HashMap;

fn extract_integers(text: &str) -> (i8, i8, i8) {
    let re = Regex::new(r"(-?\d+)").unwrap();
    let mut numbers = vec![];
    for cap in re.captures_iter(text) {
        let number_str = cap.get(1).unwrap().as_str();
        let number = number_str.parse::<i8>().unwrap();
        numbers.push(number);
    }
    (numbers[0], numbers[1], numbers[2])
}

fn main() {
    let mut f = include_str!("input.txt");
    // let mut test = include_bytes!("input.txt");

    let (raw_stacks, raw_moves) = f.split_once("\n\n").unwrap();
    // println!("{:?}", test);
    // println!("{:?}", raw_stacks.lines());
    // let mut map: HashMap<i8, &Vec<char>> = HashMap::new();
    let mut map = HashMap::new();
    let re = Regex::new(r"\d+").unwrap();

    // Parse crates
    for l in raw_stacks.lines().take(raw_stacks.lines().count() - 1) {
        println!("{}", l);
        let mut idx: i8 = 1;
        for (i, c) in l.chars().enumerate().skip(1).step_by(4) {
            if !c.is_whitespace() {
                // println!();
                // print!("{} {}",idx, c);
                let mut entry = map.entry(idx);
                entry.or_insert(vec![]).push(c);
            }
            idx += 1;
        }
        idx = 0;
    }
    for (key, value) in map.iter_mut() {
        value.reverse();
    }

    let mut map2 = map.clone();

    // println!("{:?}", map);
    // Parse instructions
    let mut instructions = Vec::new();
    for l in raw_moves.lines() {
        let numbers: (i8, i8, i8) = extract_integers(l);
        instructions.push(numbers);
    }
    println!("reshuffling crates...");

    for (n, from, to_crate) in instructions {
        let mut tmp: Vec<char> = Vec::new();
        for i in 0..n {
            // println!("{}", i);
            let value = map.get_mut(&from).unwrap().pop().unwrap();
            let value2 = map2.get_mut(&from).unwrap().pop().unwrap();
            tmp.push(value2);
            map.get_mut(&to_crate).unwrap().push(value);
        }
        tmp.reverse();
        map2.get_mut(&to_crate).unwrap().append(&mut tmp);

        // println!("{:?}", map);
        let test = 2;
    }

    let mut keys: Vec<i8> = map.keys().cloned().collect();
    keys.sort();

    // Print answer 1
    for key in keys.clone() {
        print!("{}", map[&key].last().unwrap());
    }
    println!();

    // Print answer2
    for key in keys.clone() {
        print!("{}", map2[&key].last().unwrap());
    }

}
