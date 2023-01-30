
fn main() {

    let mut f = include_str!("../../input.txt");
    let mut lines: Vec<&str> = f.split("\n").collect();
    let mut score: i32 = 0;

    for g in lines {
        let t: Vec<&str> = g.split(" ").collect();
        let o = t[0].chars().next().unwrap() as i32 - 65;
        let p = t[1].chars().next().unwrap() as i32 - 88;

        // need to lose 
        if p == 0 {
            if o % 3 == 0 {
                score += 3;
            }
            else {
                score += o % 3;
            }
        }
        // Need to draw
        else if p == 1 {
            score += 3 + o + 1;
        }
        // need to win
        else {
            score += 6 + (o + 1) % 3 + 1; 
        }
    }
    println!("{}", score);
}
