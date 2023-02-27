fn find_edge(i: i32, j: i32, dx: i32, dy: i32, target: i32, n: usize, m: usize, grid: Vec<Vec<i32>>) -> bool {

    let mut i = i;
    let mut j = j;
    let n = n as i32;
    let m = m as i32;

    while 1 == 1  {
        i += dx;
        j += dy;
        
        if i <= 0 || j <= 0 || i >= (n - 1) || j >= (m - 1) {
            return true;
        }

        if grid[i][j] >= target {
            return false;
        }


    }

return false;
}


fn main() {
    let mut text =  include_str!("input_test.txt");

    let matrix: Vec<Vec<i32>> = text
        .split('\n')
        .map(|row| {
           row.chars() 
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect()
        })
        .collect();
    println!("{:?}", matrix);

    let n = matrix.len();
    let m = matrix[0].len();
    let offset: Vec<(i32, i32)> = vec![(0, 1), (1, 0), (-1, 0), (0, -1)];

    let mut res = 0;
    for i in 0..(n - 1) {
        for j in 0..(m - 1) {
            for (dx, dy) in offset.iter() {
                let target = matrix[i][j];
                let target = target as usize;

                find_edge(i, j, (*dx).try_into().unwrap(), (*dy).try_into().unwrap(), target.try_into().unwrap(), n, m, (*matrix).to_vec()); {
                    res += 1;
                }
            }
        }
    }

    println!("{}", res);

}
