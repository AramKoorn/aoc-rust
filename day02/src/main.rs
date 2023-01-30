fn main() {

    let mut f = include_str!("../input.txt");
    let mut lines: Vec<&str> = f.split("\n").collect();
    // println!{"{:?}", lines}; println!{"{}", lines[0]};

    let mut score: i32 = 0;

    for g in lines {
        let t: Vec<&str> = g.split(" ").collect();
        // println!("{:?}", t);
        let o = t[0].chars().next().unwrap() as i32 - 65;
        let p = t[1].chars().next().unwrap() as i32 - 88;
        // println!("{:?}", p);
        let diff = p - o;
        score += p + 1;

        if diff == 0 {
            score += 3;
        }
        else if diff == 1 {
            score += 6;

        }
        else if diff == -2 {
            score += 6;
        }

        // let mut p: char = t[0];
    }
    println!("{}", score);
    // println!("{:?}", f.split("\n").map(|x| x.lines()));
     




    // println!("{:?}", f.split("\n\n").map(|x| x.lines()));
    // let mut f = f.split("\n").map(|x| x.lines());
    // for l in f {
    //     println!("{:?}", l);
    // }
    // println!{"{:?}", f};
    // println!("{}", f.len());
    // let mut l = f.lines();
    // println!("{:?}", l);
}
