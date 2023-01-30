use std::collections::HashSet;


fn main() {
    let mut f = include_str!("input.txt");
    let mut lines: Vec<&str> = f.split("\n").collect();
    // println!("{:?}", lines);

    let mut v: Vec<char> = Vec::new();

    for l in lines {
        let(mut left, mut right) = l.split_at(l.len() / 2);
        println!("{} {}", left, right);
        // println!("Common chars {}", common_chars(left, right));

        let set1: HashSet<char> = left.chars().collect();
        let set2: HashSet<char> = right.chars().collect();
        let inter: HashSet<char> = set1.intersection(&set2).cloned().collect();
        println!("{:?}", inter);

        for x in inter {
            v.push(x);
        }
    }

    let mut t: i32 = 0;

    for s in v {

        if s.is_lowercase() {
            let mut x = s as i32 - 96; 
            t += x;
            println!("{}", x);
        }
        else {
            let mut x = s as i32 - 64 + 26; 
            println!("{}", x);
            t += x;
        }
    }

    println!("{}", t);

}
