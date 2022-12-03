mod elf;
use elf::Elf;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("resources/res.txt").expect("file not found");
    let reader = BufReader::new(file);
    let content = reader.lines();
    let mut calories: Vec<i32> = Vec::new();
    let mut elf_calorie = 0;

    for line in content {
        match line {
            Ok(cont) => {
                if !cont.is_empty() {
                    elf_calorie += cont.parse::<i32>().unwrap();
                } else {
                    calories.push(elf_calorie);
                    elf_calorie = 0;
                }
            }
            Err(_) => todo!(),
        }
    }
    calories.sort();
    calories.reverse();

    println!(
        "last {:?}",
        calories.get(0).unwrap() + calories.get(1).unwrap() + calories.get(2).unwrap()
    );
}
