use std::collections::HashSet;
use std::error;
use std::fs::File;
use std::io::{self, BufRead};

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

fn get_val(ch: char) -> u32 {
    match ch {
        'A'..='Z' => ch as u8 - b'A' + 27,
        'a'..='z' => ch as u8 - b'a' + 1,
        _ => 0,
    }
    .into()
}

fn pt1() -> Result<u32> {
    let filename = "input.txt";
    let lines = io::BufReader::new(File::open(filename)?).lines();
    let mut num: u32 = 0;
    let result: Vec<String> = lines.filter_map(|x| x.ok()).collect();

    for tmp_str in result {
        let (first, second) = tmp_str.split_at(tmp_str.len() / 2);

        let first_s: HashSet<char> = first.to_string().chars().collect();
        let second_s: HashSet<char> = second.to_string().chars().collect();
        let shared: char = **first_s
            .intersection(&second_s)
            .collect::<Vec<&char>>()
            .first()
            .unwrap();
        num += get_val(shared);
    }

    Ok(num)
}

fn pt2() -> Result<u32> {
    let filename = "input.txt";
    let file = File::open(filename)?;

    let lines = io::BufReader::new(file).lines();
    let mut num: u32 = 0;
    let mut result: Vec<String> = lines.filter_map(|x| x.ok()).collect();

    while !result.is_empty() {
        let arr: Vec<String> = result.drain(0..3).collect();

        let harr: Vec<HashSet<char>> = arr
            .iter()
            .map(|x| x.to_string().chars().collect::<HashSet<char>>())
            .collect();
        let intr0: HashSet<&char> = harr[0].intersection(&harr[1]).collect();
        let intr1: HashSet<&char> = harr[1].intersection(&harr[2]).collect();
        let shared: char = **intr0.intersection(&intr1).last().unwrap();
        num += get_val(shared);
    }

    Ok(num)
}

fn main() -> Result<()> {
    println!("FINAL PT1 {:?}", pt1()?);
    println!("FINAL PT2 {:?}", pt2()?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pt1() {
        assert_eq!(pt1().unwrap(), 8105);
    }

    #[test]
    fn test_pt2() {
        assert_eq!(pt2().unwrap(), 2363);
    }
}
