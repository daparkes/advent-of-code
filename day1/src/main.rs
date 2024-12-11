use std::fs;
use std::env;

fn main() {
    let path: Vec<String> = env::args().collect();
    let nums = fs::read_to_string(&path[1])
        .expect("Failed to read file");
    
    let lines = nums.lines();
    let mut vec1: Vec<i32> = Vec::new();
    let mut vec2: Vec<i32> = Vec::new();

    for line in lines {
        let line_split: Vec<&str> = line.split("   ").collect();
        let int1: i32 = line_split[0].parse().expect("oops didn't work");
        let int2: i32 = line_split[1].parse().expect("oops didn't work");
        vec1.push(int1);
        vec2.push(int2);
    }

    vec1.sort();
    vec2.sort();
    
    let mut multipliers: Vec<i32> = Vec::new();
    for int1 in &vec1 {
        let mut count = 0;
        for int2 in &vec2 {
            if int1 == int2 {
                count += 1;
            }
        }
        multipliers.push(count);
    }

    let mut total = 0;
    let mut i = 0;
    for num in vec1 {
        let num = num * multipliers[i];
        total += num;
        i += 1;
    }

    println!("{total}");
}
