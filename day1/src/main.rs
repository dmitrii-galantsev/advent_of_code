use std::error;
use std::fs::File;
use std::io::{self, BufRead};

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

fn main() -> Result<()> {
    let filename = "input.txt";
    let file = File::open(filename)?;

    let mut index = 0;
    let mut elves: Vec<u32> = vec![0];

    for line in io::BufReader::new(file).lines() {
        let line = line?;

        if line.is_empty() {
            index += 1;
            elves.push(0);
            continue;
        }

        elves[index] += line.parse::<u32>().unwrap();
        println!("{}: {}: {}", line.is_empty(), index, line);
    }

    println!("Hello, world!");

    println!("{:?}", elves);

    elves.sort();
    elves.reverse();
    println!("pt1 answer: {:?}\n", elves[0]);
    println!("pt2 answer: {:?}\n", elves[..3].iter().sum::<u32>());

    Ok(())
}
