use std::fs;
use std::collections::HashMap;

fn main() {
    let mut inputs: [[i32; 1000]; 2] = [[0; 1000]; 2];
    let mut curr = 0;

    for line in fs::read_to_string("src/input.txt").unwrap().lines() {
        let str_vec: Vec<&str> = line.split("   ").collect();
        inputs[0][curr] = str_vec[0].parse().unwrap();
        inputs[1][curr] = str_vec[1].parse().unwrap();
        curr += 1;
    }

    merge_sort(&mut inputs[0], 0, 999);
    merge_sort(&mut inputs[1], 0, 999);
    println!("{}", similarity_score(inputs[0], inputs[1]));
}

fn similarity_score(arr1: [i32; 1000], arr2: [i32; 1000]) -> i32 {
    let mut counts: HashMap<i32, i32> = HashMap::new();
    for i in 0..arr2.len(){
        if let Some(x) = counts.get_mut(&arr2[i]) {
            *x = *x + 1;
        } else {
            counts.insert(arr2[i], 1);
        }
    }
    let mut total = 0;
    for n in arr1{
        if let Some(x) = counts.get_mut(&n) {
            total += *x * n;
        }
    }
    return total
}

fn merge_sort(arr: &mut [i32; 1000], l: usize, r: usize){
    if l < r{
        let mid: usize = ((l + r)/2) as usize;

        merge_sort(arr, l, mid);
        merge_sort(arr, mid + 1, r);
        merge(arr, l, mid, r);
    }
    //    println!("{:?}", arr);
}

fn merge(arr: &mut [i32; 1000], l: usize, m: usize, r: usize){
    let n1 = (m - l + 1) as usize;
    let n2 = (r - m) as usize;

    let mut l_vec: Vec<i32>  = vec![0; n1];
    let mut r_vec: Vec<i32> = vec![0; n2];

    for i in 0..n1{
        l_vec[i] = arr[(l + i) as usize];
    }
    for i in 0..n2{
        r_vec[i] = arr[(m + 1 + i) as usize];
    }

    let mut i: usize = 0;
    let mut j : usize = 0;
    let mut k = l as usize;

    while i < n1 && j < n2{
        if l_vec[i] <= r_vec[j]{
            arr[k] = l_vec[i];
            i += 1;
        } else {
            arr[k] = r_vec[j];
            j += 1;
        }
        k += 1;
    }

    while i < n1{
        arr[k] = l_vec[i];
        i += 1;
        k += 1;
    }

    while j < n2{
        arr[k] = r_vec[j];
        j += 1;
        k += 1;
    }
    //    println!("{:?}", arr);
}

