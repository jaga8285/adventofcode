
use std::env;
use std::fs;
use std::io::{BufRead, BufReader, Lines};

mod y2022;
mod utils;


fn parse_input(file_name: &String) -> Lines<BufReader<fs::File>>{
    
    let input_try = fs::File::open(format!("input/{}",&file_name[..file_name.len()-1]));
    let input = match input_try {
        Ok(file) => file,
        _ => {
            fs::File::open(format!("input/{}",file_name))
                .expect("Failed to read file")
        } 
    };
    let reader = BufReader::new(input);



    
    return reader.lines();
}

fn raw_input(file_name: &String) -> String{
    
    let input_try = fs::read_to_string(format!("input/{}",&file_name[..file_name.len()-1]));
    let input = match input_try {
        Ok(string) => string,
        _ => {
            fs::read_to_string(format!("input/{}",file_name))
                .expect("Failed to read file")
        } 
    };

    return input
}

/* fn read_lines<P>(filename: P) -> io::Lines<io::BufReader<File>>
where P: AsRef<Path>, {
    let file = File::open(filename);
    io::BufReader::new(file).lines()
} */



fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Not enough arguments");
    }
    let mode = &args[1];
    match mode.as_str() {
        "2a" => y2022::rps_2::rps_a(parse_input(mode)),
        "2b" => y2022::rps_2::rps_b(parse_input(mode)),
        "3a" => y2022::ruck_3::ruck_a(parse_input(mode)),
        "3b" => y2022::ruck_3::ruck_b(parse_input(mode)),
        "4a" => y2022::sect_4::sect_a(parse_input(mode)),
        "4b" => y2022::sect_4::sect_b(parse_input(mode)),
        "5a" => y2022::supp_5::supp_a(parse_input(mode)),
        "5b" => y2022::supp_5::supp_b(parse_input(mode)),
        "6a" => y2022::sign_6::sign(raw_input(mode),4),
        "6b" => y2022::sign_6::sign(raw_input(mode),14),
        "7a" => y2022::file_7::file_a(parse_input(mode)),
        "7b" => y2022::file_7::file_b(parse_input(mode)),
        "8a" => y2022::tree_8::tree_a(parse_input(mode)),
        "8b" => y2022::tree_8::tree_b(parse_input(mode)),
        "9a" => y2022::rope_9::rope(parse_input(mode), 2),
        "9b" => y2022::rope_9::rope(parse_input(mode), 10),
        "10a" => y2022::cray_10::cray_a(parse_input(mode)),
        "10b" => y2022::cray_10::cray_b(parse_input(mode)),
        bad_mode => panic!("{} is an invalid mode (a/b)", bad_mode)
    }

}
