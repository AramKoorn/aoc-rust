fn main() {
    let mut f = include_str!("input.txt");
    let mut lines: Vec<&str> = f.split("\n").collect();

    let mut res: u32 = 0;
    let mut res2: u32 = 0;
    for l in lines {
        let mut  x: Vec<&str> = l.split(",").collect();
        let mut first = x[0];
        let mut second = x[1];

        let left: Vec<i32> = first.split('-').map(|x| x.parse().unwrap()).collect();
        let right: Vec<i32> = second.split('-').map(|x| x.parse().unwrap()).collect();

        // println!("{:?} {:?}", left, right);
        if left[0] >= right[0] && left[1] <= right[1]|| right[0] >= left[0] && right[1] <= left[1]{
            res += 1;
        }
        if left[1] >= right[0] && right[1] >= left[0] {
            res2 += 1;
        }

    }
    println!("{}", res);
    println!("{}", res2);
}