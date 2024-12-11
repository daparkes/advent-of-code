use std::{env, fs};

fn main() {
    let path: Vec<String> = env::args().collect();
    let nums = fs::read_to_string(&path[1])
        .expect("Failed to read file");

    let lines = nums.lines();

    let mut total = 0;
    for line in lines {
        let line_split: Vec<&str> = line.split(" ").collect();
        let line_ints: Vec<i32> = line_split
            .into_iter()
            .map(|str| str.parse().expect("oops didn't work"))
            .collect();
        if is_safe(line_ints) {
            println!("something is safe!");
            total += 1;
        }
    }
    println!("{total}");

}

fn is_decreasing(nums: &Vec<i32>) -> bool {
    let mut comp = nums[1];

    let mut i = 1;
    for num in nums {
        if num <= &comp {
            return false;
        }
        let diff = num - comp;
        if diff < 1 || diff > 3 {
            return false;
        }
        if i + 1 < nums.len() {
            comp = nums[i + 1]
        } else {
            break;
        };
        i += 1;
    }
    true
}

fn is_increasing(nums: &Vec<i32>) -> bool {
    let mut comp = nums[1];

    let mut i = 0;
    for num in nums {
        if num >= &comp {
            return false;
        }
        let diff = comp - num;
        if diff < 1 || diff > 3 {
            return false;
        }
        if i + 1 < nums.len() {
            comp = nums[i + 1]
        } else {
            break;
        };
        i += 1;
    }
    true
}

fn is_safe(nums: Vec<i32>) -> bool {
    is_increasing(&nums) || is_decreasing(&nums)
}
