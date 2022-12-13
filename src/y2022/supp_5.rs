use regex::Regex;
use std::fs;
use std::io::{BufReader, Lines};

struct Movement{
    source: usize,
    target: usize,
    amount: u32
}
impl Movement {
    fn new(move_str: &str) -> Self {
        //I know this is bad
        let re_move = Regex::new(r"move (?P<amount>\d+) from (?P<source>\d+) to (?P<target>\d+)").unwrap();
        let caps = re_move.captures(move_str).unwrap();
        Movement {
            source: usize::from_str_radix(&caps["source"], 10).unwrap() -1,
            target: usize::from_str_radix(&caps["target"], 10).unwrap() -1,
            amount: u32::from_str_radix(&caps["amount"], 10).unwrap()
        }
    }

    fn execute(&self, stacks: &mut Vec<Stack>){
        for _id in 0..self.amount{
            let moved_char = stacks[self.source].pop();
            stacks[self.target].push(moved_char);
        }
    }
    fn execute_group(&self, stacks: &mut Vec<Stack>){
        let mut moving_chars : Vec<char> = Vec::new();
        let source_stack = &mut stacks[self.source];
        for _id in 0..self.amount{
            moving_chars.push(source_stack.pop());
        }
        moving_chars.reverse();
        let target_stack = &mut stacks[self.target];

        for char in moving_chars{
            target_stack.push(char);

        }
    }
}

#[derive(Clone)]
struct Stack {
    stack: Vec<char>
}

impl Stack {
    fn new() -> Self {
        Stack { stack: Vec::new() }
    }

    fn add(&mut self, target:char){
        if target != ' ' {
            self.stack.insert(0, target);
        }
    }
    fn push(&mut self, target:char){
        self.stack.push(target);
    }

    fn pop(&mut self) -> char{
        let res = self.stack.pop();
        res.unwrap()
    }

    
}

pub fn supp_a(lines : Lines<BufReader<fs::File>>) {
    let re_digits = Regex::new(r"\d+").unwrap();
    //let re_crates = Regex::new(r"[A-Z]").unwrap();

    let unwrapped_lines : Vec<String> = lines.map(|x| x.unwrap()).collect();

    let mut peekable_lines = unwrapped_lines.iter().peekable();
    let something = peekable_lines.peek().unwrap();
    let num_stacks = (something.len() + 1) / 4;

    let mut stacks: Vec<Stack> = vec![Stack::new(); num_stacks];


    for line in peekable_lines{
        if re_digits.is_match(&line) {
            break;
        }
        let mut char_iter = line.chars();

        for i in 0..stacks.len() {
            char_iter.next();

            if let Some(next_char) = char_iter.next(){
                if let Some(stack) = stacks.get_mut(i){
                    stack.add(next_char);
                }
            }
            char_iter.next();
            char_iter.next();
        }
    }
    peekable_lines = unwrapped_lines.iter().peekable();
    while peekable_lines.next().unwrap_or(&"".to_string()) != &"".to_string() {}
    for line in peekable_lines{
        let movement = Movement::new(&line);

        movement.execute(&mut stacks);
    }
    for mut stack in stacks {
        print!("{}",stack.stack.pop().unwrap());
    }
}
pub fn supp_b(lines : Lines<BufReader<fs::File>>) {
    let re_digits = Regex::new(r"\d+").unwrap();
    //let re_crates = Regex::new(r"[A-Z]").unwrap();

    let unwrapped_lines : Vec<String> = lines.map(|x| x.unwrap()).collect();


    let mut peekable_lines = unwrapped_lines.iter().peekable();
    let something = peekable_lines.peek().unwrap();
    let num_stacks = (something.len() + 1) / 4;

    let mut stacks: Vec<Stack> = vec![Stack::new(); num_stacks];


    for line in peekable_lines{
        if re_digits.is_match(&line) {
            break;
        }
        let mut char_iter = line.chars();

        for i in 0..stacks.len() {
            char_iter.next();

            if let Some(next_char) = char_iter.next(){
                if let Some(stack) = stacks.get_mut(i){
                    stack.add(next_char);
                }
            }
            char_iter.next();
            char_iter.next();
        }
    }
    peekable_lines = unwrapped_lines.iter().peekable();
    while peekable_lines.next().unwrap_or(&"".to_string()) != &"".to_string() {}
    for line in peekable_lines{
        let movement = Movement::new(&line);

        movement.execute_group(&mut stacks);
    }
    for mut stack in stacks {
        print!("{}",stack.stack.pop().unwrap());
    }
}