fn main() {
    // This can DEF be done better. I just start it like this for now. Adding the value
    // of each letter, and make variables of everything.... Right of the bat with an
    // improvement request for the next version :D

    let char_name = "Diggs";

    let mut total_score = 0;

    for i in char_name.chars() {
        if i == 'D' {
            total_score += 8;
        } else if i == 'i' {
            total_score += 5;
        } else if i == 'g' {
            total_score += 3;
        } else if i == 's' {
            total_score += 5;
        }
    }
    
    let last_number_str = total_score.to_string().chars().last().unwrap();
    
    println!("{}", last_number_str);
    if last_number_str == '4' {
        println!("Purplenum");
    }

    //match last_number_int {
    //    4 => println!("lksdjflksdf"),
    //}
    

}
