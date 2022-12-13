use std::{collections::HashSet, io::{Lines, BufReader}, fs};
use std::{thread, time};

#[derive(Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpRight,
    UpLeft,
    DownRight,
    DownLeft
}


impl Direction {
    fn move_pos(self, pos: &mut Position){
        match self{
            Direction::Up => pos.1 += 1,
            Direction::Down => pos.1 -= 1,
            Direction::Left => pos.0 -= 1,
            Direction::Right => pos.0 += 1,
            Direction::UpRight => {
                pos.1 += 1;
                pos.0 += 1;
            },
            Direction::UpLeft => {
                pos.1 += 1;
                pos.0 -= 1;
            },
            Direction::DownRight => {
                pos.1 -= 1;
                pos.0 += 1;
            },
            Direction::DownLeft => {
                pos.1 -= 1;
                pos.0 -= 1;
            },
        }
    }
}

struct Screen{
    max_x: i32,
    max_y: i32,
    min_x: i32,
    min_y: i32,
}

impl Screen{
    fn new() -> Self {
        Screen {max_x: 1, max_y: 1, min_x: 0, min_y: 0 }
    }

    fn render(&mut self,
        knots: &Vec<Position>,
        tail_postions: &HashSet<Position>,)
        
        {
        self.resize(knots[0]);
        for rev_y in self.min_y..self.max_y {
            let y = -rev_y + (self.max_y + self.min_y - 1);
            'render: for x in self.min_x..self.max_x {
                //print!("({},{})", x,y);
                for (i,knot) in knots.iter().enumerate() {
                    if *knot == Position(x,y){
                        print!("{}",i+1);
                        continue 'render;
                    }
                }
                if (x,y) == (0,0){
                    print!("s");
                    continue;
                }
                if tail_postions.contains(&Position(x, y)){
                    print!("#");
                    continue;
                }
                print!(".");
            }
            println!("");
        }
        println!();
        let hundred_millis = time::Duration::from_millis(200);

        thread::sleep(hundred_millis);
    }

    fn resize(&mut self,
        head: Position) {
        //println!("RESIZING FOR ({},{}): [{},{},{},{}]",head.0, head.1, self.min_x, self.max_x, self.min_y, self.max_y);
        if head.0 >= self.max_x {
            self.max_x += 1;
        }
        if head.1 >= self.max_y {
            self.max_y += 1;
        }
        if head.0 < self.min_x {
            self.min_x -= 1;
        }
        if head.1 < self.min_y {
            self.min_y -= 1;
        }
    }
}


#[derive(Eq, PartialEq, Hash, Copy, Clone)]
struct Position(i32,i32);

impl Position{
    fn adjacent(&self, other: &Position) -> bool{
        if (self.0 - other.0).abs() > 1 || (self.1 - other.1).abs() > 1{
            return false;
        }  
        true
    }
    fn approach(&self, other: &Position) -> Option<Direction> {
        if self.adjacent(other){
            return None;
        }

        if self.0 == other.0 {
            if self.1 > other.1{
                return Some(Direction::Down);
            } else {
                return Some(Direction::Up);
            }
        }

        if self.1 == other.1 {
            if self.0 > other.0{
                return Some(Direction::Left);
            } else {
                return Some(Direction::Right);
            }
        }

        if self.1 > other.1 {
            if self.0 > other.0 {
                return Some(Direction::DownLeft);
            } else {
                return Some(Direction::DownRight);
            }
        }

        if self.1 < other.1 {
            if self.0 > other.0 {
                return Some(Direction::UpLeft);
            } else {
                return Some(Direction::UpRight);
            }
        }
        return None;
    }
}

struct Rope {
    knots: Vec<Position>,
    tail_postions: HashSet<Position>,
    screen: Screen
}

impl Rope {
    fn new(size: usize) -> Self {
        let mut rope = Rope {
            knots: vec![Position(0,0); size],
            tail_postions: HashSet::new(),
            screen: Screen::new()
        };
        rope.tail_postions.insert(Position(0,0));
        rope
    }
    fn move_head(&mut self, dir: Direction, distance: u32){
        for _ in 0..distance {
            self.move_head_unit(dir);
        }
    }
    fn move_head_unit(&mut self, dir:Direction){
        dir.move_pos(&mut self.knots[0]);


        let mut knot_iter = self.knots.iter_mut();
        let mut knot_previous = knot_iter.next().unwrap();
        for knot in knot_iter{
            Rope::move_knot(knot, knot_previous);
            knot_previous = knot;
        }
        self.tail_postions.insert(*self.knots.last().unwrap());
        self.screen.render(&self.knots, &self.tail_postions);
    }

    fn move_knot(knot: &mut Position, previous_knot: &Position){{
        match knot.approach(previous_knot){
            Some(n) => n.move_pos(knot),
            None => return
        }
    }

    }
}


pub fn rope(lines : Lines<BufReader<fs::File>>, size: usize){
    let mut rope = Rope::new(size);
    for line in lines.map(|x| x.unwrap()){
        let dir = match line.chars().nth(0).unwrap(){
            'L' => Direction::Left,
            'R' => Direction::Right,
            'U' => Direction::Up,
            'D' => Direction::Down,
            _ => panic!("Invalid Input")
        };
        let distance = u32::from_str_radix(&line[2..],10).unwrap();
        rope.move_head(dir, distance);
    }
    println!("{}",rope.tail_postions.len());
}