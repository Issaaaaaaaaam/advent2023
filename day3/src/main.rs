use std::fs::File;
use std::io::{prelude::*, BufReader};

fn find_string_index(position: usize, strings: Vec<&str>) -> Option<usize> {
    let mut cumulative_len = 0;

    for (idx, s) in strings.iter().enumerate() {
        let next_cumulative_len = cumulative_len + s.len();

        if position >= cumulative_len && position < next_cumulative_len {
            return Some(idx);
        }

        cumulative_len = next_cumulative_len;
    }
    None
}
fn main() {
    let file = File::open("test_input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>().expect("shit");
    let mut sum = 0; 
    for (i, line) in lines.iter().enumerate(){
        let prev_line = lines.get(i.wrapping_sub(1));
        let next_line = lines.get(i + 1);

        let line:String = line.chars().map(|b| if !b.is_numeric(){
            vec![' ',b,' ']}
            else{vec![b]}).flatten().collect();
        let splitted:Vec<_> = line.split(" ").filter(|&b| b != " ").filter(|&b| b != "").collect();
        
        let mut indexsum:usize = 0; 
        for (zz, x) in splitted.iter().enumerate(){
            let length = &x.chars().count();
            if x.chars().next().map_or(false, |c| c=='*'){
                let start_range = if indexsum > 0 { indexsum - 1 } else { indexsum };
                let end_range = indexsum + length + 1;
                let positions: Vec<usize> = (start_range..end_range).collect();
                let mut usedprev:Vec<usize> = vec![]; 
                let mut usednext:Vec<usize> = vec![]; 
                let mut usednow:Vec<usize> = vec![]; 
                let mut nog = 0;
                let mut add_gear = 0; 
                for i in positions {
                    if !usednow.contains(&(zz-1)){
                        if true  {
                            match  splitted.get(zz-1) {
                                Some(s) if s.chars().next().expect("asba").is_numeric()  => {if add_gear == 0 
                                    { add_gear += s.parse::<u64>().expect("number")} 
                                    else {add_gear *= s.parse::<u64>().expect("number")} ;
                                         nog +=1 ;},
                                _ => ()
                            }
                        }
                    }
                    usednow.push(zz-1);
                    if !usednow.contains(&(zz+1)){
                        if true {
                            
                            match  splitted.get(zz+1) {
                                Some(s)  if s.chars().next().expect("asba").is_numeric()  =>{if add_gear == 0 
                                    { add_gear += s.parse::<u64>().expect("number")} 
                                    else {add_gear *= s.parse::<u64>().expect("number")} ;
                                         nog +=1 ;},
                                _ => ()
                            }
                        }
                    }
                    usednow.push(zz+1);
                    if prev_line != None{
                        let test2:String = prev_line.expect("assa").chars().map(|b| if !b.is_numeric(){
                            vec![' ',b,' ']}
                            else{vec![b]}).flatten().collect();
                        let kes:Vec<_> = test2.split(" ").filter(|&b| b != " ").filter(|&b| b != "").collect();
                        let astest = find_string_index(i, kes.clone()).expect("asba");
                        if !usedprev.contains(&astest) {
                            match kes.get(astest) {
                                Some(s) if s.chars().next().expect("asba").is_numeric()   => {if add_gear == 0 
                                    { add_gear += s.parse::<u64>().expect("number")} 
                                    else {add_gear *= s.parse::<u64>().expect("number")} ;
                                         nog +=1 ;},
                                _ => ()
                            }
                            usedprev.push(astest);
                        }
                    }
                    if next_line != None{
                        let test1:String = next_line.expect("assa").chars().map(|b| if !b.is_numeric(){
                            vec![' ',b,' ']}
                            else{vec![b]}).flatten().collect();
                        let kes:Vec<_> = test1.split(" ").filter(|&b| b != " ").filter(|&b| b != "").collect();
                        let astest = find_string_index(i, kes.clone()).expect("asba");
                        if !usednext.contains(&astest) {
                            match kes.get(astest){
                                Some(s) if s.chars().next().expect("asba").is_numeric()  => {if add_gear == 0 
                                                                                            { add_gear += s.parse::<u64>().expect("number")} 
                                                                                            else {add_gear *= s.parse::<u64>().expect("number")} ;
                                                                                                 nog +=1 ;},
                                _ => ()
                            }
                            usednext.push(astest);
                        }
                    }
                }
                if nog == 2 {
                    sum += add_gear;
                }           
            };
            indexsum += length;
        }
    }
    println!("The sum is {}", sum)
}

//fn main() {
//    let file = File::open("test_input.txt").unwrap();
//    let reader = BufReader::new(file);
//    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>().expect("shit");
//    let mut sum = 0; 
//    for (i, line) in lines.iter().enumerate(){
//        let prev_line = lines.get(i.wrapping_sub(1));
//        let next_line = lines.get(i + 1);
//
//        let line:String = line.chars().map(|b| if !b.is_numeric(){
//            vec![' ',b,' ']}
//            else{vec![b]}).flatten().collect();
//        let splitted:Vec<_> = line.split(" ").filter(|&b| b != " ").filter(|&b| b != "").collect();
//        
//        let mut indexsum:usize = 0; 
//        for (zz, x) in splitted.iter().enumerate(){
//            let length = &x.chars().count();
//            if x.chars().next().map_or(false, |c| c.is_numeric()){
//                let start_range = if indexsum > 0 { indexsum - 1 } else { indexsum };
//                let end_range = indexsum + length + 1;
//                let positions: Vec<usize> = (start_range..end_range).collect();
//                for i in positions {
//                    if zz + 1 < splitted.len() {
//                        let next = &splitted[zz + 1];
//                        match  next {
//                            s if !s.chars().next().expect("asba").is_numeric() && s.chars().next().expect("asba")!= '.' => {sum += x.parse::<u32>().expect("number"); break;},
//                            _ => ()
//                        }
//                    }
//                    if zz > 0 {
//                        let previous = &splitted[zz - 1];
//                        match  previous {
//                            s if !s.chars().next().expect("asba").is_numeric() && s.chars().next().expect("asba")!= '.' => {sum += x.parse::<u32>().expect("number"); break;},
//                            _ => ()
//                        }
//                    }
//                    if prev_line != None{
//                            match prev_line.expect("I cant anymore1").chars().nth(i) {
//                                Some(s) if !s.is_numeric() && s!= '.' => {sum += x.parse::<u32>().expect("number"); break;},
//                                _ => ()
//                            }
//                    }
//                    if next_line != None{
//                        match next_line.expect("I cant anymore2").chars().nth(i) {
//                            Some(s) if !s.is_numeric() && s!= '.' => sum += x.parse::<u32>().expect("number"),
//                            _ => ()
//                        }
//                    }
//                }
//            };
//            indexsum += length;
//        }
//    }
//    println!("The sum is {}", sum)
//}
//