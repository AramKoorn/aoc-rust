use std::collections::HashMap;

fn main() {
    let mut f = include_str!("input_all.txt");
    let mut commands: Vec<String> = f.lines().map(|s| s.to_string()).collect();

    let mut fs: HashMap<String, u32> = HashMap::new();
    let mut path: String = "/".to_string();

    for cmd in commands[1..commands.len()].into_iter() {
        let mut s: Vec<&str> = cmd.split(" ").collect();
        // println!{"{}", s[0]};

        if s[0] == "$" {
            if s[1] == "cd" {
                // set current path 1 directory back
                if s[2] == ".." {
                    // println!("current {}", path);
                    let mut paths: Vec<&str> = path.split("/").collect();
                    let mut slices = &paths[1..paths.len() - 2];
                    let mut new_path: String = "/".to_string();
                    for sl in slices {
                        new_path += sl;
                        new_path += "/";
                    }
                    path = new_path;
                    // println!("one back{}", path);
                } else {
                    path += s[2];
                    path += "/";
                    // println!{"{}", path};
                }
            }
        } else if s[0] != "dir" {
            let key = path.clone();
            let value = s[0].parse::<u32>().unwrap();
            let val = fs.entry(key).or_insert(0);
            *val += value;
        }
    }

    // Update child directories
    let mut system: HashMap<String, u32> = HashMap::new();
    for (p, size) in fs.iter() {
        let mut tmp = "/".to_string();
        let mut sepa: Vec<&str> = p.split("/").collect();
        for sep in sepa.iter().skip(1).take(sepa.len() - 2) {
            tmp += sep;
            let key = tmp.clone();
            // println!("{:?}", fs);
            let val = system.entry(key).or_insert(0);
            *val += size;

            println!("{}", path);
        }
    }
    println! {"System: {:?}", system};

    let mut t = 0;
    let limit: u32 = 100000;
    for (p, filesize) in system.iter() {
        if filesize <= &limit {
            println!("{}", filesize);
            t += filesize;
        }
    }

    println!("{:?}", fs);
    println!("Answer1: {}", t);
}
