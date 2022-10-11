use std::io::{self, BufRead};

fn solve(schemas: &Vec<(usize, f64)>, max_schema_size: usize, unit_price: f64, number: usize) -> f64 {
    if max_schema_size == 0 {
        return unit_price * number as f64;
    }
    let mut dp = vec![None; number + max_schema_size];
    dp[0] = Some(0.0);
    dp[1] = Some(unit_price);

    for i in 2..number + max_schema_size {
        let mut min = unit_price * i as f64;
        for (size, price) in schemas {
            if i >= *size {
                if let Some(prev) = dp[i - size] {
                    min = min.min(prev + price);
                }
            }
        }
        dp[i] = Some(min);
    }

    let mut min = dp[number].unwrap();
    for i in number + 1..number + max_schema_size {
        if let Some(prev) = dp[i] {
            min = min.min(prev);
        }
    }
    min
}

fn main() -> io::Result<()> {
    let mut case_num = 1;
    let mut buf = String::new();
    let mut handle = io::stdin().lock();
    handle.read_line(&mut buf)?;

    while buf.trim().len() > 0 {
        println!("Case {}:", case_num);
        case_num += 1;

        let mut line = buf.trim().split_whitespace();
        let unit_price = line.next().unwrap().parse::<f64>().unwrap();
        let schemas_count = line.next().unwrap().parse::<usize>().unwrap();

        let mut schemas = Vec::with_capacity(schemas_count);
        let mut max_size = 0;
        for _ in 0..schemas_count {
            buf.clear();
            handle.read_line(&mut buf)?;
            let mut line = buf.trim().split_whitespace();
            let size = line.next().unwrap().parse::<usize>().unwrap();
            if size > max_size {
                max_size = size;
            }
            let total_price = line.next().unwrap().parse::<f64>().unwrap();
            schemas.push((size, total_price));
        }

        buf.clear();
        handle.read_line(&mut buf)?;
        for number in buf.trim().split_whitespace() {
            let number = number.parse::<usize>().unwrap();
            println!("Buy {} for ${1:.2}", number, solve(&schemas, max_size, unit_price, number));
        }

        buf.clear();
        handle.read_line(&mut buf)?;
    }        

    Ok(())
}
