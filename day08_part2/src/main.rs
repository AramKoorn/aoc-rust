fn find_edge(
    i: i32,
    j: i32,
    dx: i32,
    dy: i32,
    target: i32,
    n: usize,
    m: usize,
    grid: Vec<Vec<i32>>,
) -> i32 {
    let mut i = i;
    let mut j = j;
    let n = n as i32;
    let m = m as i32;

    let mut cnt: i32 = 0;

    if i == 0 || j == 0 || i == n - 1 || j == m - 1 {
        return cnt;
    }

    loop {
        i += dx;
        j += dy;

        let row_index = i as usize;
        let col_index = j as usize;

        // check if inside the grid
        if i < 0 || j < 0 || i >= n || j >= m {
            return cnt;
        }

        if grid[row_index][col_index] >= target {
            cnt += 1;
            return cnt;
        } else {
            cnt += 1;
        }

        if i <= 0 || j <= 0 || i >= (n - 1) || j >= (m - 1) {
            return cnt;
        }
    }

    return cnt;
}

fn main() {
    let mut text = include_str!("input.txt");

    let matrix: Vec<Vec<i32>> = text
        .split('\n')
        .map(|row| {
            row.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect()
        })
        .collect();

    let n = matrix.len();
    let m = matrix[0].len();
    let offset: Vec<(i32, i32)> = vec![(0, 1), (1, 0), (-1, 0), (0, -1)];

    let mut scores: Vec<u32> = Vec::new();
    let mut mx = 1;
    for i in 0..n {
        for j in 0..m {
            let mut res = 1;
            for (dx, dy) in offset.iter() {
                let target = matrix[i][j];

                let i = i as i32;
                let j = j as i32;
                let tmp = find_edge(i, j, *dx, *dy, target, n, m, (*matrix).to_vec());
                res *= tmp;
            }
            mx = mx.max(res);
        }
    }

    println!("{}", mx);
}
