use std::collections::HashMap;

fn main() {
    let mut f = include_str!("input.txt");
    let mut commands: Vec<String> = f.lines().map(|s| s.to_string()).collect();

    let mut fs: HashMap<String, u32> = HashMap::new();
    let mut path: String = "/".to_string();

    for cmd in commands[1..commands.len() - 1].into_iter() {
        let mut s: Vec<&str> = cmd.split(" ").collect();
        // println!{"{}", s[0]};

        if s[0] == "$" {
            if s[1] == "cd" {
                // set current path 1 directory back
                if s[2] == ".." {
                    println!("current {}", path);
                    let mut paths: Vec<&str> = path.split("/").collect();
                    let mut slices = &paths[1..paths.len() - 2];
                    let mut path: String = "/".to_string();
                    for sl in slices {
                        path += sl;
                        path += "/";
                    }

                    println!("one back{}", path);
                } else {
                    path += s[2];
                    path += "/";
                    // println!{"{}", path};
                }
            }
        } else if s[0] != "dir" {
            let key = path.clone();
            println!("{:?}", s[0]);
            let value = s[0].parse::<u32>().unwrap();
            let val = fs.entry(key).or_insert(0);
            *val += value;
                    
            for sli in path.clone()[0..path.len() - 1].split("/") {
                // println!("{}", path);
                // println!("{}", sli);
            }
            
            // Update nested directories
        }
    }

    let mut t = 0;
    let limit: u32 = 100000;
    for filesize in fs.values() {
        
        if filesize < &limit{
            println!("{}", filesize);
            t += filesize;
        }
    }

    println!("{:?}", fs);
    // println!("Answer1: {}", t);



}
