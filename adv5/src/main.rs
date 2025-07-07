use std::fs;
use std::collections::{HashMap, HashSet};

fn main() {
    let mut root_node: HashMap<i32, HashSet<i32>> = HashMap::new();
    let _ = fs::read_to_string("src/input1.txt").unwrap().lines().into_iter()
        .for_each(|l| {
            let nums = l.split("|").into_iter().map(|x| x.parse().unwrap()).collect::<Vec<i32>>();
            let n1 = nums[0];
            let n2 = nums[1];
            match root_node.get_mut(&n1){
                Some(n) => {
                    n.insert(n2);
                },
                None => {
                    let mut n = HashSet::new();
                    n.insert(n2);
                    root_node.insert(n1, n);
                }
            }
        });

    let binding = fs::read_to_string("src/input2.txt").unwrap();
    let a = binding.lines().into_iter()
        .map(|l| {
            let nums = l.split(",").into_iter().map(|x| x.parse().unwrap()).collect::<Vec<i32>>();
            return nums
        })
        .filter(|nums| {
            for i in 0..nums.len()-1{
                for j in i+1..nums.len(){
                    match root_node.get(&nums[j]){
                        Some(node) => {
                            if node.contains(&nums[i]){
                                return true
                            }
                        },
                        None => {}
                    }
                }
            }
            return false
            
        })
        .map(|nums| {
//            println!("{:?}", nums);
//          return nums

            let mut new_nums: Vec<i32> = vec![nums[0]];
            for i in 1..nums.len(){
                let mut inserted = false;
                for j in 0..new_nums.len(){
                    match root_node.get(&nums[i]){
                        Some(node) => {
                            if node.contains(&new_nums[j]){
                                inserted = true;
                                new_nums.insert(j, nums[i]);
                                break
                            }
                        }, 
                        None => {}
                    }
                }
                if !inserted {
                    new_nums.push(nums[i]);
                }
            } 
            return new_nums
        })
        .into_iter();
    let b: i32 = a.clone().map(|nums| {
            return nums[nums.len()/2]
        })
        .reduce(|a, b| a + b).expect("WHY");
    println!("{:?}", b);
}
