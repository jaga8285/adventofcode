use std::fs;
use std::io::{BufReader, Lines};

struct Range {
    start: u32,
    end: u32
}

impl Range {
    fn contains_range(&self, other : &Range) -> bool{
        self.start <= other.start && self.end >= other.end
    }


    fn overlaps(&self, other : &Range) -> bool{
        self.start <= other.end && self.end >= other.start
    }
    
}

fn get_ranges(line : &String) -> (Range,Range){
    let split_line : Vec<&str> = line.split(",").collect();
    let first_range_str : Vec<&str> = split_line[0].split("-").collect();
    let second_range_str : Vec<&str> = split_line[1].split("-").collect();
    let first_range = Range {
        start: u32::from_str_radix(first_range_str[0], 10).expect("Invalid number"),
        end: u32::from_str_radix(first_range_str[1], 10).expect("Invalid number") };
    let second_range = Range {
        start: u32::from_str_radix(second_range_str[0], 10).expect("Invalid number"),
        end: u32::from_str_radix(second_range_str[1], 10).expect("Invalid number") };
    return ((first_range), second_range)
}


pub fn sect_a(lines : Lines<BufReader<fs::File>>) {
    let mut total = 0;
    for line in lines.map(|x| x.unwrap()){
        let ranges = get_ranges(&line);
        if ranges.0.contains_range(&ranges.1) ||ranges.1.contains_range(&ranges.0) {
            total = total + 1;
        }
    }
    println!("{}", total);
}

pub fn sect_b(lines : Lines<BufReader<fs::File>>) {
    let mut total = 0;
    for line in lines.map(|x| x.unwrap()){
        let ranges = get_ranges(&line);
        if ranges.0.overlaps(&ranges.1) {
            //println!("{},{} {} >= {}",ranges.0.start,ranges.1.end,ranges.0.end,ranges.1.start);
            total = total + 1;
        }
    }
    println!("{}", total);
}