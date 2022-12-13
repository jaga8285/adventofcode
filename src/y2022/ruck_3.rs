use std::collections::HashSet;
use std::fs;
use std::io::{BufReader, Lines};

fn calc_char_score(c:char) -> u32 {
    if c.is_lowercase(){
        (c as u32) - 96 
    } else {
        (c as u32) - 64 +26
    }
}

pub fn ruck_a(lines : Lines<BufReader<fs::File>>) {
    let mut total = 0;
    for line in lines.map(|x| x.unwrap()) {
       let (sack1, sack2) = line.split_at(line.len()/2); 
       let mut sack1_contents = HashSet::new();
       for char in sack1.chars() {
           sack1_contents.insert(char);
       }
       for char in sack2.chars() {
           if sack1_contents.contains(&char){
               total = total + calc_char_score(char);
               break;
           }
       }
    }
    println!("{}",total);
}

pub fn ruck_b(mut lines : Lines<BufReader<fs::File>>) {
    let mut total = 0;
    while let (Some(Ok(sack_a)), Some(Ok(sack_b)), Some(Ok(sack_c))) = (lines.next(), lines.next(), lines.next()) {
       let mut sack_a_contents = HashSet::new();
       let mut sack_b_contents = HashSet::new();
       for char in sack_a.chars() {
           sack_a_contents.insert(char);
       }
       for char in sack_b.chars() {
           sack_b_contents.insert(char);
       }
       for char in sack_c.chars() {
           if sack_a_contents.contains(&char) && sack_b_contents.contains(&char){
               dbg!(char);
               total = total + calc_char_score(char);
               break;
           }
       }
    }
    println!("{}",total);
}
