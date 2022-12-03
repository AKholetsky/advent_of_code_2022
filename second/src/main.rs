use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let result = run("resources/release.txt");
    println!("{}", result);
}

fn run(file: &str) -> i32 {
    let file = File::open(file).expect("can't open file");
    let content = BufReader::new(file).lines();
    let mut final_result = 0;
    for line in content {
        let s = line.unwrap();
        let mut i = s.split_whitespace();
        let letters: (&str, &str) = (i.next().unwrap(), i.next().unwrap());
        let f_res = fight_result(letters.0, letters.1); // A Y = 0
        let point_of_win = point_for_win(f_res); // 0 -> 3
        let points_for_letter = get_value(letters.0, f_res); // A 0 -> 1
        let result_of_fight_and_choose = point_of_win + points_for_letter;
        final_result += result_of_fight_and_choose;
    }

    final_result
}

fn get_value(figure: &str, fight_result: i32) -> i32 {
    match figure {
        "A" => match_with_a(fight_result),
        "B" => match_with_b(fight_result),
        "C" => match_with_c(fight_result),
        _ => 4,
    }
}

fn fight_result(first: &str, second: &str) -> i32 {
    match first {
        "A" => fight(second),
        "B" => fight(second),
        "C" => fight(second),
        _ => -22,
    }
}

fn fight(second: &str) -> i32 {
    match second {
        "X" => -1,
        "Y" => 0,
        "Z" => 1,
        _ => 22,
    }
}

fn match_with_a(letter: i32) -> i32 {
    match letter {
        -1 => 3,
        0 => 1,
        1 => 2,
        _ => 22,
    }
}

fn match_with_b(letter: i32) -> i32 {
    match letter {
        -1 => 1,
        0 => 2,
        1 => 3,
        _ => 22,
    }
}

fn match_with_c(letter: i32) -> i32 {
    match letter {
        -1 => 2,
        0 => 3,
        1 => 1,
        _ => 22,
    }
}

fn point_for_win(result: i32) -> i32 {
    match result {
        -1 => 0,
        0 => 3,
        1 => 6,
        _ => 100,
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_run() {
        use crate::run;
        let result: i32 = run("resources/test.txt");
        assert_eq!(result, 12)
    }
}
