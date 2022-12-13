use std::fmt;

use regex::Regex;
use lazy_static::lazy_static;
use regex::RegexSet;
use regex::RegexSetBuilder;
use crate::utils::Tree;
use std::fs;
use std::io::{BufReader, Lines};


#[derive(Debug)]
enum Command{
    CD(String),
    LS
}


#[derive(Debug)]
enum TermLine{
    Output(Node),
    Cmd(Command)
}

struct Node{
    name: String,
    size: Option<u32>,
    file: bool,
    checked: bool, //Assuming the folders or files aren't created mid input
    max_size : u32
}

impl Node {
    fn new_dir(name: &str, max_size: u32) -> Self{
        Node { 
            name: String::from(name),
            size: Some(0),
            file: false,
            checked: false,
            max_size
        }
    }
    fn new_file(name: &str, initial_size: u32, max_size:u32) -> Self{
        Node { 
            name: String::from(name),
            size: {
                if initial_size > max_size{
                    None
                } else {
                    Some(initial_size)
                }
            }, 
            file:true,
            checked: false,
            max_size
        }
    }

    fn add(&mut self, rhs:Option<u32>){
        if self.size == None || rhs == None {
            self.size = None;
            return;
        }

        match self.size.unwrap().checked_add(rhs.unwrap()){
            Some(n) => {
                if n > self.max_size{
                    self.size = None;
                } else {
                    self.size = Some(n);
                }
            },
            None => self.size = None,
        }
    }
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name;
        let size;
        if self.file {
            name = "File"
        } else {
            name = "Dir"
        }
        if let Some(n) = self.size{
            size = n.to_string();
        } else {
            size = String::from("OVERFLOW");
        }
        f.debug_struct(name)
         .field("size", &size)
         .finish()
    }
}

fn parse_line(line: &str, max_size: u32) -> TermLine{
    lazy_static! {
        static ref RE_SET: RegexSet = RegexSetBuilder::new(&[
            r"^\$ cd (.+)$",     //cd
            r"^\$ ls$",          //ls
            r"^dir (.+)$",       //dir
            r"^([0-9]+) (.+)$"    //file
        ]).build().expect("Regex Set couldn't be built");
        static ref RE_CD : Regex = Regex::new(r"^\$ cd (.+)$").unwrap();
        static ref RE_LS : Regex = Regex::new(r"^\$ ls$").unwrap();
        static ref RE_DIR : Regex = Regex::new(r"^dir (.+)$").unwrap();
        static ref RE_FILE : Regex = Regex::new(r"^([0-9]+) (.+)$").unwrap();
    }
    let matches = RE_SET.matches(line);
    match matches.iter().next().expect(format!("Couldn't match Regex, Line: {}", line).as_str()) {
        0 =>{
                let parsed = RE_CD
                    .captures(line).unwrap()
                    .get(1).unwrap()
                    .as_str();
                TermLine::Cmd(Command::CD(String::from(parsed)))
        },
        1 =>TermLine::Cmd(Command::LS),
        2 =>{
            let parsed = RE_DIR
                .captures(line).unwrap()
                .get(1).unwrap()
                .as_str();
            TermLine::Output(Node::new_dir(parsed, max_size))
        },
        3 =>{
            let caps = RE_FILE
                .captures(line).unwrap();
            let size = caps.get(1).unwrap().as_str();
            let name = caps.get(2).unwrap().as_str();
            TermLine::Output(
                Node::new_file(
                    name, 
                    u32::from_str_radix( size, 10).unwrap(),
                    max_size
                )
            )
        },
        _ => panic!("Invalid regex match. Line: {}", line)

    }
    
}

fn dfs_size_calc(tree: &mut Tree<Node>, index: usize){
    for child in tree.get_children(index).clone(){
        let child_node;
        {
            child_node = tree.get_data(child);
            if !child_node.file{
                dfs_size_calc(tree, child);
            }
        }
        let child_size = tree.get_data(child).size;
        tree.get_mut_data(index).add(child_size);
    }
    
}

pub fn file_a(lines : Lines<BufReader<fs::File>>){
    let max_size = 100_000;
    let mut tree = Tree::new(Node::new_dir("/", max_size ));
    let mut current_node_id = 0;
    for line in lines.map(|x| x.unwrap()){
        // dbg!(&tree);
        // dbg!(&line);
        match parse_line(&line, max_size){
            TermLine::Output(node) => {
                if tree.get_data(current_node_id).checked {
                    continue;
                }
                tree.add(current_node_id, node);
            },
            TermLine::Cmd(Command::CD(name)) => 
            {
                current_node_id = match name.as_str() {
                    "/" => 0,
                    ".." => tree.get_parent(current_node_id).expect("Couldn't find parent node"),
                    dir => {
                        let mut found_node_id = 0;
                        for child in tree.get_children(current_node_id){
                            // println!("{} ?= {}", tree.get_data(*child).name, dir);

                            if tree.get_data(*child).name == dir{
                                found_node_id = *child;
                                break;
                            }
                        }

                        found_node_id
                    }
                }
            },
            TermLine::Cmd(Command::LS) => continue,
        } 
    }
    dfs_size_calc(&mut tree, 0);
    let mut total = 0;
    for node in tree.get_iter(){
        if let Some(size) = node.size{
            if !node.file{
                total += size;
            }
        }
    }
    tree.pretty_print();
    println!("{}", total);
}

pub fn file_b(lines : Lines<BufReader<fs::File>>){
    let max_size = 1_000_000_000;
    let mut tree = Tree::new(Node::new_dir("/", max_size ));
    let mut current_node_id = 0;
    for line in lines.map(|x| x.unwrap()){
        // dbg!(&tree);
        // dbg!(&line);
        match parse_line(&line, max_size){
            TermLine::Output(node) => {
                if tree.get_data(current_node_id).checked {
                    continue;
                }
                tree.add(current_node_id, node);
            },
            TermLine::Cmd(Command::CD(name)) => 
            {
                current_node_id = match name.as_str() {
                    "/" => 0,
                    ".." => tree.get_parent(current_node_id).expect("Couldn't find parent node"),
                    dir => {
                        let mut found_node_id = 0;
                        for child in tree.get_children(current_node_id){
                            // println!("{} ?= {}", tree.get_data(*child).name, dir);

                            if tree.get_data(*child).name == dir{
                                found_node_id = *child;
                                break;
                            }
                        }

                        found_node_id
                    }
                }
            },
            TermLine::Cmd(Command::LS) => continue,
        } 
    }
    dfs_size_calc(&mut tree, 0);

    let needed_space = tree.get_data(0).size.unwrap() - 40_000_000;

    let mut smallest_file_size = u32::MAX;


    for node in tree.get_iter(){
        if let Some(size) = node.size{
            if !node.file && size > needed_space && size < smallest_file_size{
                smallest_file_size = size;
            }
        }
    }
    tree.pretty_print();
    println!("{}", smallest_file_size);
}