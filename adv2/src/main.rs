use std::fs;

fn main() {
    let mut inputs: [Vec<i32>; 1000] = std::array::from_fn(|_| vec![]); 
    let mut curr = 0;

    for line in fs::read_to_string("src/input.txt").unwrap().lines() {
        inputs[curr] = line.split(" ").map(|x| x.parse().unwrap()).collect();
        curr += 1;
    }

    let mut total = 0;
    for i in 0..1000{
        let current_vec = inputs[i].clone();
        let mut asc = false; 
        let mut asces = 0;
        if current_vec[1] - current_vec[0] > 0{
            asces += 1;
        }
        if current_vec[2] - current_vec[1] > 0{
            asces += 1;
        }
        if current_vec[3] - current_vec[2] > 0{
            asces += 1;
        }
        if asces > 1{
            asc = true
        }
        if check_validity(&current_vec, asc, false){
            total += 1
        }
    }

    println!("{:?}", total);


}

fn check_validity(current_vec: &Vec<i32>, asc: bool, used: bool) -> bool{
    for j in 1..current_vec.len(){
        let c = current_vec[j] - current_vec[j-1];
        if c == 0 || i32::abs(c) > 3 || (!asc && c > 0) || (asc && c < 0) {
            
            let mut copy1 = current_vec.clone();
            copy1.remove(j-1);
            let mut copy2 = current_vec.clone();
            copy2.remove(j);

            if !used && (check_validity(&copy1, asc, true) || check_validity(&copy2, asc, true)){
               return true;                 
            }

            return false;
        }
    }
    return true;
}
