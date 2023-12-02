use std::fs::File;
use std::io::{prelude::*, BufReader};

///part 2
fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut sum:u32 = 0; 
    for lines in reader.lines(){
        let mut min_blue:u32 = 0;
        let mut min_red:u32 =0; 
        let mut min_green:u32 =0;
        let line = lines.expect("Problem with line"); 
        let game = line.split_once(":").expect("Could not find :").1; 
        //let game_number:u32 = game_number.split_once(" ").expect("No space to split at").1.parse().expect("No number");
        let game:Vec<_> = game[1..].split(" ").collect();
        //println!("{:?}", &game);
        for (a, b) in game.iter().zip(game.iter().skip(1)) {
            match b {
                b if b.contains("b") => if a.parse::<u32>().expect("number")> min_blue {min_blue = a.parse::<u32>().expect("number")},
                b if b.contains("d") => if a.parse::<u32>().expect("number")> min_red {min_red = a.parse::<u32>().expect("number")},
                b if b.contains("g") => if a.parse::<u32>().expect("number")> min_green {min_green = a.parse::<u32>().expect("number")},
                _ => ()
            }
        //println!("a is: {}\nb is :{}", &a, &b);
        }
        sum += min_blue *min_red* min_green
    }
    println!("The sum is: {}", sum)
}

/* PART 1 ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
use std::fs::File;
use std::io::{prelude::*, BufReader};
fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut sum:u32 = 0; 
    for lines in reader.lines(){
        let mut isok:bool = true;
        let line = lines.expect("Problem with line"); 
        let (game_number, game) = line.split_once(":").expect("Could not find :"); 
        let game_number:u32 = game_number.split_once(" ").expect("No space to split at").1.parse().expect("No number");
        let game:Vec<_> = game[1..].split(" ").collect();
        //println!("{:?}", &game);
        for (a, b) in game.iter().zip(game.iter().skip(1)) {
            match b {
                b if b.contains("b") => if a.parse::<usize>().expect("number")> 14 {isok = false},
                b if b.contains("d") => if a.parse::<usize>().expect("number")> 12 {isok = false},
                b if b.contains("g") => if a.parse::<usize>().expect("number")> 13 {isok = false},
                _ => ()
            }
        //println!("a is: {}\nb is :{}", &a, &b);
        }
        if isok == true {sum += game_number}
    }
    println!("The sum is: {}", sum)
}
*/

