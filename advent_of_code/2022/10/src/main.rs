use std::io;

enum Instruction {
    Addx(isize),
    Noop
}

impl Instruction {
    fn from_str(input: &str) -> Self {
        let mut parts = input.split_whitespace();
        let command = parts.next().unwrap();
        match command {
            "addx" => {
                let value = parts.next().unwrap().parse::<isize>().unwrap();
                Instruction::Addx(value)
            },
            "noop" => Instruction::Noop,
            _ => panic!("Unknown command: {}", command)
        }
    }
}

fn main() {
    let mut current = 1;
    let mut registers = Vec::new();
    registers.push(current);

    for line in io::stdin().lines() {
        let line = line.unwrap();
        let instruction = Instruction::from_str(&line);
        match instruction {
            Instruction::Addx(value) => {
                for _ in 0..2 {
                    registers.push(current);
                }
                current += value;
            },
            Instruction::Noop => {
                registers.push(current);
            }
        }
    }

    let mut signal_strength = 0;
    for cycle in [20, 60, 100, 140, 180, 220].iter() {
        signal_strength += registers[*cycle] * (*cycle as isize);
    }
    println!("{}", signal_strength);
}
