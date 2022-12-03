use std::collections::HashSet;
use std::error;
use std::fs::File;
use std::io::{self, BufRead};

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

fn get_val(ch:char) -> u8 {
    match ch {
        'A'..='Z' => ch as u8 - b'A' + 27,
        'a'..='z' => ch as u8 - b'a' + 1,
        _ => 0
    }
}

fn pt1() -> Result<()> {
    let filename = "input.txt";
    let file = File::open(filename)?;

    let lines = io::BufReader::new(file).lines();
    let mut num: u32 = 0;
    let result: Vec<String> = lines.filter_map(|x| x.ok()).collect();
    println!("{:?}", result);

    for tmp_str in result {
        //let tmp_str = &result[0];
        let split = tmp_str.split_at(tmp_str.len()/2);
        let (first, second) = split;
        //let (first_s, second_s) = (HashSet::from(first), HashSet::from(second));
        //let intrsct = first.intersection(&second);
        println!("{:?} {:?}", first, second);
        println!("{:?}", first.to_string().as_bytes());

        println!("{:?}", get_val('p'));
        println!("{:?}", get_val('L'));

        let first_s: HashSet<char> = first.to_string().chars().collect();
        let second_s: HashSet<char> = second.to_string().chars().collect();
        println!("{:?}", first_s.intersection(&second_s));
        println!("{:?}", first_s.intersection(&second_s));
        let shared: char = **first_s.intersection(&second_s).collect::<Vec<&char>>().first().unwrap();
        println!("{:?}", get_val(shared));
        num += u32::from(get_val(shared));
    }

    println!("FINAL PT1 {:?}", num);

    Ok(())
}

fn pt2() -> Result<()> {
    let filename = "input.txt";
    let file = File::open(filename)?;

    let lines = io::BufReader::new(file).lines();
    let mut num: u32 = 0;
    let mut result: Vec<String> = lines.filter_map(|x| x.ok()).collect();
    println!("{:?}", result);

    while !result.is_empty() {
        let arr: Vec<String> = result.drain(0..3).collect();
        println!("{:?}", arr[0]);

        let harr: Vec<HashSet<char>> = arr.iter().map(|x| x.to_string().chars().collect::<HashSet<char>>()).collect();
        println!("{:?}", harr[0].intersection(&harr[1]));
        println!("{:?}", &harr[1].intersection(&harr[2]));
        let intr0: HashSet<&char> = harr[0].intersection(&harr[1]).collect();
        let intr1: HashSet<&char> = harr[1].intersection(&harr[2]).collect();
        //let shared: char = intr0.clone().intersection(&intr1).collect::<HashSet<&&char>>().len();
        let shared = intr0.intersection(&intr1).collect::<HashSet<&&char>>();
        let shared: char = ***shared.iter().next().unwrap();
        println!("{:?}", get_val(shared));
        num += u32::from(get_val(shared));
    }

    println!("FINAL PT2 {:?}", num);

    Ok(())
}

fn main() -> Result<()> {
    pt1()?;
    pt2()
}
