// // Actual input
const MONKEYS_COUNT: usize = 8;

// Sample input
// const MONKEYS_COUNT: usize = 4;

const ROUNDS: usize = 10000;

// // Actual input
const DIVIDER: isize = 9699690;

// Sample input
// const DIVIDER: isize = 96577;

fn main() {
    let mut monkeys: Vec<Vec<isize>> = Vec::new();

    // Actual input
    monkeys.push(Vec::from([71, 86]));
    monkeys.push(Vec::from([66, 50, 90, 53, 88, 85]));
    monkeys.push(Vec::from([97, 54, 89, 62, 84, 80, 63]));
    monkeys.push(Vec::from([82, 97, 56, 92]));
    monkeys.push(Vec::from([50, 99, 67, 61, 86]));
    monkeys.push(Vec::from([61, 66, 72, 55, 64, 53, 72, 63]));
    monkeys.push(Vec::from([59, 79, 63]));
    monkeys.push(Vec::from([55]));

    // Sample input
    // monkeys.push(Vec::from([79, 98]));
    // monkeys.push(Vec::from([54, 65, 75, 74]));
    // monkeys.push(Vec::from([79, 60, 97]));
    // monkeys.push(Vec::from([74]));


    let mut inspected = [0; MONKEYS_COUNT];

    for _ in 0..ROUNDS {
        for i in 0..MONKEYS_COUNT {
            let levels = monkeys[i].clone();
            match i {
                // Sample input
                // 0 => {
                //     for level in levels {
                //         let level = (level * 19) % DIVIDER;
                //         if level % 23 == 0 {
                //             monkeys[2].push(level);
                //         } else {
                //             monkeys[3].push(level);
                //         }
                //         inspected[i] += 1;
                //     }
                // },
                // 1 => {
                //     for level in levels {
                //         let level = (level + 6) % DIVIDER;
                //         if level % 19 == 0 {
                //             monkeys[2].push(level);
                //         } else {
                //             monkeys[0].push(level);
                //         }
                //         inspected[i] += 1;
                //     }
                // },
                // 2 => {
                //     for level in levels {
                //         let level = (level * level) % DIVIDER;
                //         if level % 13 == 0 {
                //             monkeys[1].push(level);
                //         } else {
                //             monkeys[3].push(level);
                //         }
                //         inspected[i] += 1;
                //     }
                // },
                // 3 => {
                //     for level in levels {
                //         let level = (level + 3) % DIVIDER;
                //         if level % 17 == 0 {
                //             monkeys[0].push(level);
                //         } else {
                //             monkeys[1].push(level);
                //         }
                //         inspected[i] += 1;
                //     }
                // },
                // Actual input
                0 => {
                    for level in levels {
                        let level = level * 13 % DIVIDER;
                        if level % 19 == 0 {
                            monkeys[6].push(level);
                        } else {
                            monkeys[7].push(level);
                        }
                        inspected[i] += 1;
                    }
                },
                1 => {
                    for level in levels {
                        let level = (level + 3) % DIVIDER;
                        if level % 2 == 0 {
                            monkeys[5].push(level);
                        } else {
                            monkeys[4].push(level);
                        }
                        inspected[i] += 1;
                    }
                },
                2 => {
                    for level in levels {
                        let level = (level + 6) % DIVIDER;
                        if level % 13 == 0 {
                            monkeys[4].push(level);
                        } else {
                            monkeys[1].push(level);
                        }
                        inspected[i] += 1;
                    }
                },
                3 => {
                    for level in levels {
                        let level = (level + 2) % DIVIDER;
                        if level % 5 == 0 {
                            monkeys[6].push(level);
                        } else {
                            monkeys[0].push(level);
                        }
                        inspected[i] += 1;
                    }
                },
                4 => {
                    for level in levels {
                        let level = level * level % DIVIDER;
                        if level % 7 == 0 {
                            monkeys[5].push(level);
                        } else {
                            monkeys[3].push(level);
                        }
                        inspected[i] += 1;
                    }
                },
                5 => {
                    for level in levels {
                        let level = (level + 4) % DIVIDER;
                        if level % 11 == 0 {
                            monkeys[3].push(level);
                        } else {
                            monkeys[0].push(level);
                        }
                        inspected[i] += 1;
                    }
                },
                6 => {
                    for level in levels {
                        let level = (level * 7) % DIVIDER;
                        if level % 17 == 0 {
                            monkeys[2].push(level);
                        } else {
                            monkeys[7].push(level);
                        }
                        inspected[i] += 1;
                    }
                },
                7 => {
                    for level in levels {
                        let level = (level + 7) % DIVIDER;
                        if level % 3 == 0 {
                            monkeys[2].push(level);
                        } else {
                            monkeys[1].push(level);
                        }
                        inspected[i] += 1;
                    }
                },
                _ => panic!("Unknown monkey: {}", i)
            }
            monkeys[i].clear();
        }
    }

    println!("Inspected: {:?}", inspected);
    println!("{}", 145312 as i128 * 145314 as i128);
}
