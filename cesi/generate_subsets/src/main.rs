use std::io;

fn search(k: i32, n: i32, subset: &mut Vec<i32>) {
    if k == n + 1 {
        for elm in subset {
            print!("{} ", elm);
        }
        println!("");
    } else {
        subset.push(k);
        search(k + 1, n, subset);
        subset.pop();
        search(k + 1, n, subset);
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: i32 = input.trim().parse().unwrap();
    let mut subset: Vec<i32> = Vec::new();

    search(1, n, &mut subset);
}
