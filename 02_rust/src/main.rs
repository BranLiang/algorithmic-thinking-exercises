use std::io;
use std::collections::HashSet;

fn is_compound(word: &str, dict: &HashSet<String>) -> bool {
    for i in 1..word.len() + 1 {
        if dict.contains(&word[..i]) && dict.contains(&word[i..]) {
            return true;
        }
    }
    false
}

fn main() -> io::Result<()> {
    let mut words = HashSet::new();
    let mut compound_words = Vec::new();

    let mut buf = String::new();

    // Load words from stdin
    loop {
        buf.clear();
        io::stdin().read_line(&mut buf)?;
        if buf.is_empty() {
            break;
        }
        words.insert(buf.trim().to_string());
    }

    // Checking if there is compound word
    for word in words.iter() {
        if is_compound(word, &words) {
            compound_words.push(word);
        }
    }

    compound_words.sort();
    for word in compound_words.iter() {
        println!("{}", word);
    }

    Ok(())
}
