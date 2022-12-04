use std::fs;

pub fn day01() {
    let file_path = "inputs/01.txt";

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    process_text_day01(&contents);
}

fn process_text_day01(contents: &str) {
    let mut max_cal_vec = vec![0i32, 0i32, 0i32];
    let mut max_cals:i32 = 0;
    let mut current_cals:i32 = 0;
    for line in contents.lines() {
        if !line.is_empty() {
            current_cals += line.parse::<i32>().unwrap();
        } else {
            if current_cals > max_cals {
                max_cals = current_cals;
            }
            max_cal_vec.sort();
            if max_cal_vec[0] < current_cals {
                max_cal_vec[0] = current_cals;
            }
            current_cals = 0;
        }
    }
    println!("Day01: The maximum calories are {}", max_cals);
    println!("Day01: The number of calories top 3 is {}", max_cal_vec.iter().sum::<i32>());
}

pub fn day02() {
    let file_path = "inputs/02.txt";

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    process_text_day02(&contents);
}

fn process_text_day02(contents: &str) {
    // vars
    let mut final_result = 0;
    let rock = 1;
    let paper = 2; 
    let scissors = 3;

    for line in contents.lines() {
        let mut hand = line.split(" ");
        let _oponent = hand.next().unwrap();
        let played_hand = hand.next().unwrap();
        match played_hand {
            "X" => final_result += rock,
            "Y" => final_result += paper,
            "Z" => final_result += scissors,
            _ => (),
        }
        final_result += get_hand_points(line);

    }
    println!("Day02: The final restult according to plan is {}", final_result);
}

fn get_hand_points(line: &str) -> i32 {
    let mut play = line.split(" ");
    let oponent = play.next().unwrap();
    let mine = play.next().unwrap();
    
    if oponent.eq("A") {
        match mine {
            "X" => return 3,
            "Y" => return 6,
            "Z" => return 0,
            _ => return 0,
        }
    }
    if oponent == "B" {
        match mine {
            "X" => return 0,
            "Y" => return 3,
            "Z" => return 6,
            _ => return 0,
        } 
    }
    if oponent == "C" {
        match mine {
            "X" => return 6,
            "Y" => return 0,
            "Z" => return 3,
            _ => return 0,
        } 
    }
    return 0i32;
}