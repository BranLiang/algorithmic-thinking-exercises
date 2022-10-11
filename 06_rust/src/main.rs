use std::io;
use std::cmp;

fn solve(m: usize, n: usize, t: usize) {
    let mut dp = vec![-2; t + 1];
    dp[0] = 0;
    for i in 1..=t {
        let mut first = -1;
        let mut second = -1;
        if i >= m {
            first = dp[i - m];
        }
        if i >= n {
            second = dp[i - n];
        }
        if first >= 0 || second >= 0 {
            dp[i] = 1 + cmp::max(first, second);
        } else {
            dp[i] = -1;
        }
    }
        
    let result = dp[t];
    if result < 0 {
        let mut i = 1;
        while dp[t - i] < 0 {
            i += 1;
        }
        println!("{} {}", dp[t - i], i);
    } else {
        println!("{}", result);
    }
}

fn main() -> io::Result<()> {
    for line in io::stdin().lines() {
        if let Some(line) = line.ok() {
            let mut iter = line.trim().split_whitespace();
            let m = iter.next().unwrap().parse::<usize>().unwrap();
            let n = iter.next().unwrap().parse::<usize>().unwrap();
            let t = iter.next().unwrap().parse::<usize>().unwrap();

            solve(m, n, t);
        }
    }

    Ok(())
}
