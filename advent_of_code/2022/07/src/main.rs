use std::{io, collections::HashMap};

#[derive(Debug)]
struct File {
    name: String,
    size: usize,
}

#[derive(Debug)]
struct Dir {
    files: Vec<File>,
    dirs: Vec<String>,
}

impl Dir {
    fn size(&self, fs: &FileSystem) -> usize {
        let mut size = 0;
        for file in &self.files {
            size += file.size;
        }
        for dir in &self.dirs {
            match fs.get(dir) {
                Some(dir) => size += dir.size(fs),
                None => size += 0,
            }
        }
        size
    }

    fn parse(current_path: &str, outputs: &Vec<String>) -> Self {
        let mut files = Vec::new();
        let mut dirs = Vec::new();
        for output in outputs {
            let mut parts = output.split_whitespace();
            let part = parts.next().unwrap();
            match part {
                "dir" => {
                    let dir = parts.next().unwrap();
                    let mut path = current_path.to_string();
                    path.push('/');
                    path.push_str(dir);
                    dirs.push(path);
                },
                _ => {
                    let size = part.parse::<usize>().unwrap();
                    let name = parts.next().unwrap();
                    files.push(File { name: name.to_string(), size });
                }
            }
        }
        Self { files, dirs }
    }
}

type FileSystem = HashMap<String, Dir>;

fn main() {
    let mut current_path = Vec::new();
    let mut fs = FileSystem::new();

    let mut ls_outputs = Vec::new();
    for line in io::stdin().lines() {
        let line = line.unwrap();
        let mut parts = line.split_whitespace();
        let part = parts.next().unwrap();
        match part {
            "$" => {
                if ls_outputs.len() > 0 {
                    let dir = Dir::parse(&current_path.join("/"), &ls_outputs);
                    let path = current_path.join("/");
                    fs.insert(path, dir);
                }

                let command = parts.next().unwrap();
                match command {
                    "ls" => {
                        ls_outputs.clear();
                    },
                    "cd" => {
                        let dir = parts.next().unwrap();
                        if dir == ".." {
                            current_path.pop();
                        } else {
                            current_path.push(dir.to_string());
                        }
                        ls_outputs.clear();
                    },
                    _ => panic!("Unknown command: {}", command)
                }
            },
            _ => {
                ls_outputs.push(line);
            }
        }
    }

    if ls_outputs.len() > 0 {
        let dir = Dir::parse(&current_path.join("/"), &ls_outputs);
        let path = current_path.join("/");
        fs.insert(path, dir);
    }

    let mut sum = 0;
    for dir in fs.values() {
        let size = dir.size(&fs);
        if size <= 100000 {
            sum += size;
        }
    }
    println!("part 1: {}", sum);

    // Part 2
    let total = 70000000;
    let root_size = fs.get("/").unwrap().size(&fs);
    let needed = 30000000 - (total - root_size);

    let mut minimum = 0;
    for dir in fs.values() {
        let size = dir.size(&fs);
        if size >= needed {
            if minimum == 0 || size < minimum {
                minimum = size;
            }
        }
    }

    println!("part 2: {}", minimum);
}
