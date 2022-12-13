use std::collections::VecDeque;
use std::collections::HashSet;


fn test_signal(signal : &VecDeque<char>) -> bool {
    let mut contained_chars: HashSet<char> = HashSet::new();
    for char in signal {
        if !contained_chars.insert(*char) {
            return false;
        }
    }
    true
}

pub fn sign(input: String, marker_len: usize) {
    let char_iter = input.chars();
    let mut signal_suffix: VecDeque<char> = VecDeque::new();
    for (i,char) in char_iter.enumerate(){
        if signal_suffix.len() > marker_len {
            let _ = &signal_suffix.pop_front();
            if test_signal(&signal_suffix){
                println!("{}",i);
                return;
            }
        }
        let _ = &signal_suffix.push_back(char);
    }
    
}