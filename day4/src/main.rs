use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::collections::HashMap;




fn recursive_score(tabs:&HashMap<usize, (i32,i32)>, key:usize, score:&mut i32){
    let (_points, mut count) = tabs.get(&key).expect("AAAA THE DICT FAILED").clone();
    //println!("I am at key {} adding score {}", &key, &points);
    *score += 1;
    //println!("The score is: {}", score);
    while count>0 {
        recursive_score(tabs, key + count as usize, score);
        count -= 1;
    }
}

fn main() {
    let file = File::open("test_input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>().expect("shit");
    let mut overwall_wins = 0; 
    let mut tabs:HashMap<usize, (i32,i32)> = HashMap::new();
    for (i, line) in lines.iter().enumerate(){
        let interest:String = line.split_once(":").expect("could not split at : ").1.to_string(); 
        let (winning,number) = interest.split_once("|").expect("could not split at : ");
        let winning:Vec<_> = winning.split(" ").filter(|&b| b != "").collect();
        let number:Vec<_> = number.split(" ").filter(|&b| b != "").collect();
        let mut current_card_points =0; 
        let mut amount_of_win_num = 0; 
        for win in &winning {
            for num in &number {
                if win == num {
                    if current_card_points == 0{
                        current_card_points += 1
                    }
                    else {
                        current_card_points *= 2
                    }
                    amount_of_win_num += 1;
                }
            }
        }
        tabs.insert(i, (current_card_points,amount_of_win_num));
        //overwall_wins += current_card_points;
        //println!("Winning: {:?}\nNumbers: {:?}", &winning, &number)
    }
    for (key, _value) in &tabs {
        recursive_score(&tabs, *key, &mut overwall_wins);
        //println!("{} / {:?}", key, value);
    }
    println!("total: {}", overwall_wins)
}
