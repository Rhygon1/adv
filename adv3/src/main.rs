use regex::Regex;
use std::fs;

fn main() {
    let re = Regex::new(r"(mul\(\d?\d?\d,\d?\d?\d\)|(do\(\))|(don\'t\(\)))").unwrap();
    let re2 = Regex::new(r"(\d?\d?\d,\d?\d?\d)").unwrap();
    let mut enabled = true;
    
    let input = fs::read_to_string("src/input.txt").unwrap();
    let dates: Vec<i32> = re.find_iter(&input)
        .map(|m| {
            let s = m.as_str();
            if s == "do()"{
                enabled = true;                 
            } else if s == "don't()"{
                enabled = false;
            } else if enabled {
                let Some(caps) = re2.find(s) else {
                    return 0 as i32;
                };
                let binding = caps.as_str();
                let spl = binding.split(",").collect::<Vec<&str>>();
                let mul: i32 = spl[0].parse::<i32>().unwrap()*spl[1].parse::<i32>().unwrap();
                return mul as i32;
            }
            return 0 as i32;
        })
        .collect();
let sum = dates.iter().cloned().reduce(|a, b| a + b).unwrap_or(0);
println!("{:?}", sum);
}
