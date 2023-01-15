use std::io;

fn generate(n: i32, permutation: &mut Vec<i32>, used: &mut Vec<bool>) {
    if permutation.len() == n as usize {
        println!("{:?}", permutation);
    } else {
        for i in 1..=n {
            if !used[i as usize] {
                used[i as usize] = true;
                permutation.push(i);
                generate(n, permutation, used);
                used[i as usize] = false;
                permutation.pop();
            }
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: i32 = input.trim().parse().unwrap();

    let mut permutation: Vec<i32> = Vec::new();
    let mut used: Vec<bool> = vec![false; n as usize + 1];

    generate(n, &mut permutation, &mut used);
}
