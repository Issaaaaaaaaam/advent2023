use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut sum:usize = 0; 
    for line in reader.lines() {
        let char_vec: Vec<char> = line.expect("Something happened in the lines").replace("one", "o1ne")
                                                            .replace("two", "t2wo")
                                                            .replace("three", "th3ree")
                                                            .replace("four", "f4our")
                                                            .replace("five", "f5ive")
                                                            .replace("six", "s6ix")
                                                            .replace("seven", "sev7en")
                                                            .replace("eight", "eig8ht")
                                                            .replace("nine", "ni9ne")
                                                            .replace("ten", "ze0ro").chars().collect();
        //println!("{:?}", char_vec);
        let mut number:String= Default::default(); 
        for character in char_vec {
            match character { 
                '1'..='9' =>  {
                    if number.is_empty(){
                        number.push_str(&character.to_string());
                        number.push_str(&character.to_string());
                    }
                    else {
                        number = number[..1].to_string();
                        number.push_str(&character.to_string());
                    }
                }
                _ => ()
            }
        }
        if number.len() == 1 {
        }
        println!("{}", number);
        sum += number.parse::<usize>().expect("Not a number")
    }
    println!("Sum: {}", sum)
}
