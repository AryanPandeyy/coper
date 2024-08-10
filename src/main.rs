use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::collections::HashMap;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut file = File::open(&args[1])?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;

    // if it contains, increase the count
    // if it doesnt add and count = 0

    let mut freq: HashMap<char, i32> = HashMap::new();

    for  letter in buffer.chars().map(|x| x) {
        if freq.contains_key(&letter) {
            let mut count = *freq.get(&letter).unwrap();
            count += 1;
            freq.insert(letter, count);
        } else {
            if letter.is_ascii_alphabetic() {
                freq.insert(letter, 1);
            }
        }
    }

    println!("{:?}", freq);

    Ok(())
}
