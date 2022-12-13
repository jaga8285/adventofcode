use std::io::{BufReader, Lines};
use std::fs;



fn parse_trees(lines : Lines<BufReader<fs::File>>) -> Vec<Vec<u8>> {
    let mut matrix : Vec<Vec<u8>> = Vec::new();
    for line in lines.map(|x| x.unwrap()) {
        let mut row = Vec::<u8>::new();
        for char in line.chars(){
            row.push(char.to_digit(10).unwrap() as u8);
        }
        matrix.push(row);
    }
    matrix
}


fn check_visibility(forest: &Vec<Vec<u8>>, coordinates : (usize, usize)) -> bool{
    let tree = forest[coordinates.0][coordinates.1];
    let width = forest.first().unwrap().len();
    let height = forest.len();

    //horizontal traversal from left
    for j in 0..width{
        let neighbor_tree = forest[coordinates.0][j];
        if  j == coordinates.1 {
            return true;
        }
        if neighbor_tree >= tree {
            break;
        }
    }
    
    //horizontal traversal from right
    for rev_j in 0..width{
        let j = width - rev_j - 1;
        let neighbor_tree = forest[coordinates.0][j];
        if  j == coordinates.1 {
            return true;
        }
        if neighbor_tree >= tree {
            break;
        }
    }
    
    //vertical traversal from top
    for i in 0..height{
        let neighbor_tree = forest[i][coordinates.1];
        if i == coordinates.0 {
            return true;
        }
        if neighbor_tree >= tree {
            break;
        }
    }
    
    //vertical traversal from bottom
    for rev_i in 0..height{
        let i = height - rev_i - 1;
        let neighbor_tree = forest[i][coordinates.1];
        if i == coordinates.0 {
            return true;
        }
        if neighbor_tree >= tree {
            break;
        }
    }
    false

}

fn get_scenic_score(forest: &Vec<Vec<u8>>, coordinates : (usize, usize), debug:bool) -> u32{
    let width = forest.first().unwrap().len();
    let height = forest.len();

    let (mut right_scenic_score, mut down_scenic_score, mut left_scenic_score, mut up_scenic_score) = (0,0,0,0);
    let mut debug_str = String::new();
    let tree = forest[coordinates.0][coordinates.1];

    //look right
    for j in coordinates.1..width-1{
        debug_str.push('r');
        let neighbor_tree = forest[coordinates.0][j+1];
        //println!("Comparing {} with {} ({},{})", tree, neighbor_tree, coordinates.0, j);
        right_scenic_score += 1;
        if neighbor_tree >= tree {
            break;
        }
    }

    //look left
    for j in (1..coordinates.1+1).rev(){        
        debug_str.push('l');
        let neighbor_tree = forest[coordinates.0][j-1];
        left_scenic_score += 1;
        if neighbor_tree >= tree {
            break;
        }
    }
    

    //look down
    for i in coordinates.0..height-1{
        debug_str.push('d');
        let neighbor_tree = forest[i+1][coordinates.1];
        down_scenic_score += 1;
        if neighbor_tree >= tree {
            break;
        }
    }
    

    //look up
    for i in (1..coordinates.0+1).rev(){
        debug_str.push('u');
        let neighbor_tree = forest[i-1][coordinates.1];
        up_scenic_score += 1;
        if neighbor_tree >= tree {
            break;
        }
    }
    let scenic_score = right_scenic_score * up_scenic_score * down_scenic_score * left_scenic_score;

    debug_str.push_str(&(scenic_score).to_string());
    debug_str.push_str(&format!("({},{})", coordinates.0, coordinates.1));
    if debug{
        println!("{}",debug_str);
    }
    scenic_score
}

pub fn tree_a(lines : Lines<BufReader<fs::File>>){
    let forest= parse_trees(lines);

    let width = forest.first().unwrap().len();
    let height = forest.len();

    let mut visible_tree_count = 0;
    for i in 0..height{
        for j in 0..width{
            if i == 0 || j == 0 || i == height - 1 || j == width - 1{
                visible_tree_count += 1;
                continue;
            }
            if check_visibility(&forest, (i,j)) {
                visible_tree_count += 1;
            }
        }

    }
    println!("{}",visible_tree_count);
}

pub fn tree_b(lines : Lines<BufReader<fs::File>>){
    let forest= parse_trees(lines);

    let width = forest.first().unwrap().len();
    let height = forest.len();

    let mut max_scenic_score = 0;
    for i in 1..height-1{
        for j in 1..width-1{
            let scenic_score = get_scenic_score(&forest, (i,j), false);
            if scenic_score > max_scenic_score {
                max_scenic_score = scenic_score;
            }
        }
    }
    println!("{}",max_scenic_score);
}

