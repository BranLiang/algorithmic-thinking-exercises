use std::io;

fn parse_r_c() -> (usize, usize) {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let r = iter.next().unwrap().parse().unwrap();
    let c = iter.next().unwrap().parse().unwrap();
    (r, c)
}

#[derive(Debug)]
enum Cell {
    Wall,
    Start,
    Cake,
    Block,
}

fn main() {
    let (r, _) = parse_r_c();
    let mut grid = Vec::new();

    for _ in 0..r {
        let mut row = Vec::new();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        for cell in input.trim().chars() {
            match cell {
                '#' => row.push(Cell::Wall),
                'S' => row.push(Cell::Start),
                'C' => row.push(Cell::Cake),
                '.' => row.push(Cell::Block),
                _ => panic!("Invalid cell"),
            }
        }
        grid.push(row);
    }

    println!("{:?}", grid);
}
