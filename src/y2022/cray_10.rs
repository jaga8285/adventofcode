use std::{io::{Lines, BufReader}, fs};

enum Command{
    Addx(i32),
    Noop
}

impl Command{
    fn parse(raw_string: &String) -> Self{
        let mut args = raw_string.split(" ");
        match args.next().unwrap(){
            "noop" => Command::Noop,
            "addx" => {
                let amount = i32::from_str_radix(args.next()
                    .unwrap(), 10)
                    .unwrap();
                Command::Addx(amount)
            },
            _ => panic!("Invalid command {}", raw_string)
        }
    }
}

struct CPU {
    x_reg : i32,
    mem: Vec<i32>,
    rendering: bool
}

impl CPU {
    fn new(rendering: bool) -> CPU{
        CPU{
            x_reg: 1,
            mem: vec![1],
            rendering
        }
    }
    fn exec_cmd(&mut self, cmd: Command){
        match cmd{
            Command::Addx(n) => {
                self.clock();
                self.clock();
                self.x_reg += n;
            }
            Command::Noop => self.clock(),
        }
    }
    fn clock(&mut self){
        self.render();
        self.mem.push(self.x_reg);
    }

    fn render(&self){
        if !self.rendering {
            return;
        }
        let rendered_char ;
        let clock = self.mem.len() -1;
        let coords = (clock % 40, clock / 40);
        if self.x_reg.abs_diff((coords.0) as i32) <= 1 {
            rendered_char = '#';
        } else {
            rendered_char = '.';
        }
        if (clock+1) % 40 == 0{
            println!("{}",rendered_char);
        } else {
            print!("{}",rendered_char);
        }
    }
}

pub fn cray_a(lines : Lines<BufReader<fs::File>>){
    let mut cpu = CPU::new(false);
    let mut unwrapped_lines = lines.map(|x| x.unwrap());
    while cpu.mem.len() < 220{
        let cmd = Command::parse(&unwrapped_lines.next().unwrap());
        cpu.exec_cmd(cmd);
    }
    let mut res  = 0;
    for i in (20..=220).step_by(40){
        println!("{} * {}",i, cpu.mem[i]);
        res += (i as i32)*cpu.mem[i];
    }
    println!("{}",res);
}


pub fn cray_b(lines : Lines<BufReader<fs::File>>){
    let mut cpu = CPU::new(true);
    let unwrapped_lines = lines.map(|x| x.unwrap());
    for line in unwrapped_lines{
        let cmd = Command::parse(&line);
        cpu.exec_cmd(cmd);
    }
}