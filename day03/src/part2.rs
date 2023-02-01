use std::collections::HashSet;

pub fn inter(x: &[&str]) -> i32 {

    let set1: HashSet<char> = x[0].chars().collect();
    let set2: HashSet<char> = x[1].chars().collect();
    let set3: HashSet<char> = x[2].chars().collect();
    let over: HashSet<char> = set1.intersection(&set2).cloned().collect();
    let over2: HashSet<char> = over.intersection(&set3).cloned().collect();

    let vec: Vec<char> = over2.into_iter().collect();
    let mut s:char = vec[0];


    let mut ans = 0;
    if s.is_lowercase() {
        ans = s as i32 - 96; 
    }
    else {
         ans= s as i32 - 64 + 26; 
    }
    return ans;
    // let tmp = x[0]; println!("{:?}", tmp);
}


pub fn part2() {
    let mut f = include_str!("input.txt");
    let mut lines: Vec<&str> = f.split("\n").collect();
    // println!("{:?}", lines);

    let mut v: Vec<char> = Vec::new();
    // println!("{:?}", lines);

    let chunks = lines.chunks(3).map(|x| inter(x)).collect::<Vec<i32>>();
    println!("{:?}", chunks);
    let sum: i32 = chunks.iter().sum();
    println!("Answer part2: {}", sum);
}
