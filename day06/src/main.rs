use std::collections::HashSet;

fn fn_unique(s: &str, k: usize) {
    for i in 0..(s.len() - k) {
        let tmp = &s[i..i + k];
        let s = tmp.chars().collect::<HashSet<_>>();

        if s.len() == k {
            println!("{}", i + k);
            break;
        }
    }
}

fn main() {
    let mut f: &str = include_str!("input.txt");
    println!("{}", f);

    fn_unique(f, 4);
    fn_unique(f, 14);
}
