use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;
use std::time::Instant;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = &args[1];
    let mut file_name = "./src/input.txt";
    if file == "test" {
        file_name = "./src/input1.txt";
    }
    let binding = fs::read_to_string(file_name).unwrap();

    let input: HashMap<String, HashSet<String>> = binding
        .lines()
        .into_iter()
        .map(|l| {
            let nodes: Vec<String> = l.split("-").into_iter().map(|l| l.to_string()).collect();
            return [nodes[0].clone(), nodes[1].clone()];
        })
        .fold(
            HashMap::new(),
            |mut acc: HashMap<String, HashSet<String>>, x: [String; 2]| {
                match acc.get_mut(&x[0]) {
                    Some(node) => {
                        node.insert(x[1].to_string());
                    }
                    None => {
                        acc.insert(x[0].clone(), HashSet::from([x[1].to_string()]));
                    }
                }
                match acc.get_mut(&x[1]) {
                    Some(node) => {
                        node.insert(x[0].to_string());
                    }
                    None => {
                        acc.insert(x[1].clone(), HashSet::from([x[0].to_string()]));
                    }
                }
                return acc;
            },
        );

    let mut max = 0;
    let mut biggest: Vec<String> = vec![];
    let mut count = 0;
    let mut existing: HashMap<String, usize> = HashMap::new();
    let start = Instant::now();
    let col = input.iter();
    let mut vec_input: Vec<(&String, &HashSet<String>)> = col.collect();
    vec_input.sort_by_key(|w| w.1.len());
    
    for n in vec_input {
        println!("{}/{}, {}", count, input.len(), n.0.clone());
        count += 1;
        let m = build_lan(
            &HashSet::from([n.0.clone()]),
            &n.1,
            &input,
            max,
            &mut existing,
        );
        if m > max {
            max = m;
            biggest = vec![n.0.clone()];
        } else if max == m {
            biggest.push(n.0.clone());
        }
    }

    biggest.sort_by_key(|s| turn_into_number(s.to_string()));
    println!(
        "{:?}, {}, {}",
        biggest,
        max,
        biggest
            .clone()
            .into_iter()
            .fold("".to_string(), |acc, x| acc + &x.to_string() + ",")
    );
    println!("{:?}", start.elapsed());
}

fn build_lan(
    curr_group: &HashSet<String>,
    curr_next: &HashSet<String>,
    input: &HashMap<String, HashSet<String>>,
    max: usize,
    cache: &mut HashMap<String, usize>,
) -> usize {
    if curr_next.len() == 0 {
        return curr_group.len();
    }

    let mut max_length = 0;
    for node in curr_next {
        let mut curr_group_copy = curr_group.clone();
        curr_group_copy.insert(node.clone());
        let mut ordered_group: Vec<usize> = curr_group_copy
            .iter()
            .map(|s| {
                return turn_into_number(s.to_string());
            })
            .collect();

        ordered_group.sort();
        let map_string = ordered_group
            .into_iter()
            .fold("".to_string(), |acc, x| acc + &x.to_string() + "-");
        match cache.get(&map_string) {
            Some(v) => {
                return *v;
            }
            None => {}
        }

        match input.get(node) {
            Some(cons) => {
                let curr_next_copy: HashSet<String> = cons
                    .intersection(curr_next)
                    .cloned()
                    .collect::<HashSet<String>>();
                if curr_next_copy.len() + curr_group_copy.len() < usize::max(max_length, max) {
                    continue;
                }
                let l = build_lan(
                    &curr_group_copy,
                    &curr_next_copy,
                    input,
                    usize::max(max_length, max),
                    cache,
                );
                cache.insert(map_string, l);
                if l > max_length {
                    max_length = l;
                }
            }
            None => {}
        }
    }
    return max_length;
}

fn turn_into_number(s: String) -> usize {
    let mut letter_to_number = HashMap::new();
    for (i, letter) in ('a'..='z').enumerate() {
        letter_to_number.insert(letter, i + 1);
    }

    let mut repr: usize = 0;
    match s.chars().nth(0) {
        Some(c) => match letter_to_number.get(&c) {
            Some(x) => repr += x * 28,
            None => {}
        },
        None => {}
    }
    match s.chars().nth(1) {
        Some(c) => match letter_to_number.get(&c) {
            Some(x) => repr += x,
            None => {}
        },
        None => {}
    }
    return repr;
}
