#[derive(Debug)]
struct Point1 { 
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Point { 
    x: i32,
    y: i32,
    visited: Vec<Point1>
}

#[derive(Debug)]
enum Direction {
    Up(i32),
    Down(i32),
    Left(i32), 
    Right(i32)
    
}

fn update_tail(tail: &mut Point, head: &Point) {

    // Calculate euclid distance
    let t_x = tail.x as f32;
    let t_y: f32 = tail.y as f32;
    let h_x: f32 = head.x as f32;
    let h_y: f32 = head.y as f32;

    let distance = ((t_x - h_x).powi(2) + (t_y - h_y).powi(2)).sqrt();

    if distance >= 2.0 {
        println!("This is a test do not panic!");

        // share x ax
        if tail.x == head.x {
            tail.y = (tail.y + head.y) / 2;
            println!("tail is going to be{:?}", tail);
        }
        if tail.y == head.y {
            tail.x = (tail.x + head.x) / 2;
            println!("tail is going to be{:?}", tail);
        }

    }
}

fn main() {
    let mut inp: Vec<&str> = include_str!("input_test.txt").lines().collect();
    println!("{:?}", inp);

    // start at (0, 0)
    let mut head: Point = Point { x: 0, y: 0, visited: vec![]};
    let mut tail: Point = Point { x: 0, y: 0, visited: vec![]};
    head.visited.push(Point1 { x: 0, y: 0 });
    tail.visited.push(Point1 { x: 0, y: 0 });

    println!("{:?}", head);

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
                for i in 1..steps {
                    head.y += 1;
                    update_tail(&mut tail, &head);
                }
            }
            Direction::Down(steps) => {
                for i in 1..steps {
                    head.y += -1;
                    update_tail(&mut tail, &head);
                }
            }
            Direction::Left(steps) => {
                for i in 1..steps {
                    head.x += -1;
                    update_tail(&mut tail, &head);
                }
            }
            Direction::Right(steps) => {
                for i in 1..steps {
                    head.x += 1;
                    update_tail(&mut tail, &head);
                }
            }
        }

        
    }



}
