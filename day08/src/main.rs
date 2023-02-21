fn main() {
    let mut text =  include_str!("input_test.txt");

    let matrix: Vec<Vec<u32>> = text
        .split('\n')
        .map(|row| {
           row.chars() 
                .map(|c| c.to_digit(10).unwrap())
                .collect()
        })
        .collect();
    println!("{:?}", matrix);

    let n = matrix.len();
    let m = matrix[0].len();

    for i in 0..n{
        for j in 0..m{
            println!("{}", matrix[i][j]);
        }
    }

}
