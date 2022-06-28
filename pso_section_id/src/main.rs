// TODO, make a logic, that will suggest the correct array with items. 

use std::io::stdin;

fn main() {

    // PRESENT THE OPTIONS
    let section_ids = ["Virdia", "Greenill", "Skyly", "Bluefull", "Purplenum", "Pinkal", "Redria", "Oran", "Yellowboz", "Whithill"];
    for id in section_ids.iter().enumerate() {
        println!("{:?}", id);
    }

    // WHICH ONE YOU AIMING FOR? (0-9)
    println!("Which one are you going to make (0-9): ");
    let mut choice = String::new();
    stdin().read_line(&mut choice)
        .ok()
        .expect("Fail");

    match choice.as_str().trim() {
        "0" => println!("{}", section_ids[0]),
        "1" => println!("{}", section_ids[1]),
        "2" => println!("{}", section_ids[2]),
        "3" => println!("{}", section_ids[3]),
        "4" => println!("{}", section_ids[4]),
        "5" => println!("{}", section_ids[5]),
        "6" => println!("{}", section_ids[6]),
        "7" => println!("{}", section_ids[7]),
        "8" => println!("{}", section_ids[8]),
        "9" => println!("{}", section_ids[9]),
        _ => println!("Sorry, no match"),
    };

    // TAKE CHARACTER NAME
    println!("Enter your desired name to calculate the Section ID: ");
    let mut char_name = String::new();
    stdin().read_line(&mut char_name)
        .ok()
        .expect("Fail");

    // LOGIC TO CONVERT NAME INTO A SCORE
    let mut total_score = 0;

    let zer = ['F', 'P', 'Z', 'd', 'n', 'x', '2', '(', '<'];
    let one = ['G', 'Q', 'e', 'o', 'y', '3', ')', '=', '['];
    let two = ['H', 'R', 'f', 'p', 'z', '4', '*', '\\', '>', ' '];
    let thr = ['I', 'S', 'g', 'q', '5', '!', '+', '{', ']', '?'];
    let fou = ['J', 'T', 'h', 'r', '6', '@', '^', '}', '"', ',']; 
    let fiv = ['K', 'U', 'i', 's', '7', '#', '-', '_', '|'];
    let six = ['B', 'L', 'V', 'j', 't', '8', '~', '$', '\'', '.'];
    let sev = ['C', 'M', 'W', 'a', 'k', 'u', '9', '%', '/'];
    let eig = ['D', 'N', 'X', 'b', 'l', 'v', '0', '&', ':'];
    let nin = ['E', 'O', 'Y', 'c', 'm', 'w', '1', '`', ';'];

    for c in char_name.chars() {
        if zer.contains(&c) {
            total_score += 0;
        } else if one.contains(&c) {
            total_score += 1;
        } else if two.contains(&c) {
            total_score += 2;
        } else if thr.contains(&c) {
            total_score += 3;
        } else if fou.contains(&c) {
            total_score += 4;
        } else if fiv.contains(&c) {
            total_score += 5;
        } else if six.contains(&c) {
            total_score += 6;
        } else if sev.contains(&c) {
            total_score += 7;
        } else if eig.contains(&c) {
            total_score += 8;
        } else if nin.contains(&c) {
            total_score += 9;
        }
    }

    // CALCULATE THE SECTION IDs
    // Array of Section ids
//    let section_ids = ["Virdia", "Greenill", "Skyly", "Bluefull", "Purplenum", "Pinkal", "Redria", "Oran", "Yellowboz", "Whithill"];
    
    // Since the Section ID is based off the LAST digit in the number: 
    // Get last digit to determine the ID
    let last_num = total_score % 10;

    println!("Name: {}\nTotal score: {}\nLast digit: {}", char_name, total_score, last_num);
    println!("\nSection-ID: {}", section_ids[last_num]);    
}
