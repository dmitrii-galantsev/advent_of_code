use std::error;
use std::fs::File;
use std::io::{self, BufRead};

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

fn main() -> Result<()> {
    let filename = "input.txt";
    let file = File::open(filename)?;

    let lines = io::BufReader::new(file).lines();

    let result_pt1: Vec<u32> = lines.filter_map(|x| x.ok()).map(|x|
        match x.as_str() {
            "A X" => 4,
            "B X" => 1,
            "C X" => 7,
            "A Y" => 8,
            "B Y" => 5,
            "C Y" => 2,
            "A Z" => 3,
            "B Z" => 9,
            "C Z" => 6,
            _ => 100000000
        }
    ).collect();

    println!("{:?}", result_pt1);
    println!("result: {:?}\n", result_pt1.into_iter().sum::<u32>());

    let file = File::open(filename)?;
    let lines = io::BufReader::new(file).lines();

    let result_pt2: Vec<u32> = lines.filter_map(|x| x.ok()).map(|x|
        match x.as_str() {
            "A X" => 3,
            "B X" => 1,
            "C X" => 2,
            "A Y" => 4,
            "B Y" => 5,
            "C Y" => 6,
            "A Z" => 8,
            "B Z" => 9,
            "C Z" => 7,
            _ => 100000000
        }
    ).collect();

    println!("{:?}", result_pt2);
    println!("result2: {:?}", result_pt2.into_iter().sum::<u32>());

    Ok(())
}
