use std::collections::HashSet;

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
struct Point1 {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone)]
struct Point {
    x: i32,
    y: i32,
    visited: Vec<Point1>,
}

impl Point {
    pub fn init() -> Point {
        return Point {
            x: 0,
            y: 0,
            visited: vec![Point1 { x: 0, y: 0 }],
        };
    }

    pub fn init_knots(n: usize) -> Vec<Point> {
        return vec![Point::init(); n];
    }
}

#[derive(Debug)]
enum Direction {
    Up(i32),
    Down(i32),
    Left(i32),
    Right(i32),
}

fn update_tail(knots: &mut Vec<Point>, direction: Direction) {
    for i in (1..10).rev() {
        // let mut tail = &mut knots[i - 1];
        let n: usize = i;

        // Calculate euclid distance
        let t_x = knots[n - 1].x as f32;
        let t_y: f32 = knots[n - 1].y as f32;
        let h_x: f32 = knots[n].x as f32;
        let h_y: f32 = knots[n].y as f32;

        let distance = ((t_x - h_x).powi(2) + (t_y - h_y).powi(2)).sqrt();
        // println!("{} {} {} {}", t_x, t_y, h_x, h_y);
        // println!("Distance: {}", distance);

        if distance >= 2.0 {
            // share x-axes
            if knots[n - 1].y == knots[n].y {
                knots[n - 1].x = (knots[n - 1].x + knots[n].x) / 2;
                let x = knots[n - 1].x;
                let y = knots[n - 1].y;
                knots[n - 1].visited.push(Point1 { x: x, y: y });
                // continue;
            }

            if knots[n - 1].x == knots[n].x {
                knots[n - 1].y = (knots[n - 1].y + knots[n].y) / 2;
                let x = knots[n - 1].x;
                let y = knots[n - 1].y;
                knots[n - 1].visited.push(Point1 { x: x, y: y });
                // continue;
            }

            if knots[n].x > knots[n - 1].x && knots[n].y > knots[n - 1].y {
                knots[n - 1].x += 1;
                knots[n - 1].y += 1;
            }
            if knots[n].x > knots[n - 1].x && knots[n].y < knots[n - 1].y {
                knots[n - 1].x += 1;
                knots[n - 1].y -= 1;
            }
            if knots[n].x < knots[n - 1].x && knots[n].y < knots[n - 1].y {
                knots[n - 1].x -= 1;
                knots[n - 1].y -= 1;
            }
            if knots[n].x < knots[n - 1].x && knots[n].y > knots[n - 1].y {
                knots[n - 1].x -= 1;
                knots[n - 1].y += 1;
            }
            let x = knots[n - 1].x;
            let y = knots[n - 1].y;
            knots[n - 1].visited.push(Point1 { x: x, y: y });
        }
    }
}

fn main() {
    let mut inp: Vec<&str> = include_str!("input.txt").lines().collect();

    let n: usize = 10;
    let mut rope: Vec<Point> = Point::init_knots(n);

    for x in inp {
        let splitted = x.split(" ");
        let (letter, number) = match x.split_whitespace().collect::<Vec<&str>>()[..] {
            [letter, number] => (letter, number),
            _ => panic!("Invalid input format"),
        };
        let steps = number.parse::<i32>().unwrap();

        let direction = match letter {
            "R" => Direction::Right(steps),
            "D" => Direction::Down(steps),
            "L" => Direction::Left(steps),
            "U" => Direction::Up(steps),
            _ => panic!("No direction"),
        };

        match direction {
            Direction::Up(steps) => {
                for _ in 0..steps {
                    rope[n - 1].y += 1;
                    update_tail(&mut rope, Direction::Up(steps));
                }
            }
            Direction::Down(steps) => {
                for _ in 0..steps {
                    rope[n - 1].y += -1;
                    update_tail(&mut rope, Direction::Up(steps));
                }
            }
            Direction::Left(steps) => {
                for _ in 0..steps {
                    rope[n - 1].x += -1;
                    update_tail(&mut rope, Direction::Up(steps));
                }
            }
            Direction::Right(steps) => {
                for _ in 0..steps {
                    rope[n - 1].x += 1;
                    update_tail(&mut rope, Direction::Up(steps));
                }
            }
        }
    }

    // println!("number of points: {}", tail.visited.len());
    println!("visited total: {:?}", rope[0].visited.len());

    let v: &Vec<Point1> = &rope[0].visited;
    let unique_set: HashSet<_> = v.into_iter().collect();
    let num_unique = unique_set.len();

    println!("Answer part2: {}", num_unique);
}
