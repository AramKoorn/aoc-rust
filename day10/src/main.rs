#[derive(Debug)]
enum Instructions {
    Noop,
    Addx(i32),
}

fn update_cycle(n: i32, mut cnt: i32, v: i32, mut s: i32) -> (i32, i32) {
    let signal = vec![20, 60, 100, 140, 180, 220];
    for _ in 1..n + 1 {
        cnt += 1;
        if signal.contains(&cnt) {
            let upd = v * cnt;
            s += upd;
        }
    }
    return (cnt, s);
}

fn main() {
    let mut f = include_str!("input.txt");
    let mut input: Vec<Instructions> = f
        .lines()
        .map(|x| match x.trim() {
            "noop" => Instructions::Noop,
            s if s.starts_with("addx") => {
                let value = s[4..].trim().parse().unwrap_or(0);
                Instructions::Addx(value)
            }
            _ => Instructions::Noop,
        })
        .collect();
    println!("{:?}", input);

    let mut cnt: i32 = 0;
    let mut v: i32 = 1;
    let mut prev = 0;
    let mut signal = 0;

    for i in input {
        v += prev;

        match i {
            Instructions::Addx(n) => {
                prev = n;
                (cnt, signal) = update_cycle(2, cnt, v, signal);
            }
            Instructions::Noop => {
                prev = 0;
                (cnt, signal) = update_cycle(1, cnt, v, signal);
            }
        }
    }
    println!("{}", signal);
}
