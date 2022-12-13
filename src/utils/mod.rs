use std::{collections::HashSet, fmt::Debug};

#[derive(Debug)]
pub struct Tree<T> {
   nodes: Vec<T>,
   children: Vec<HashSet<usize>>, //each position in outer vec represents a node. The Set for each node contains all it's children
   node_count: usize
}

impl<T> Tree<T> 
where T: Debug{
    pub fn new(root: T) -> Self {
        let mut tr =Tree {
            nodes: Vec::new(),
            children: vec![],
            node_count: 0
        };
        tr.nodes.push(root);
        tr.children.push(HashSet::new());
        tr.node_count += 1;
        tr
    }

    pub fn add(&mut self, parent_id: usize, node: T){
        let child_id = self.node_count;
        //println!("Adding {} to {}", child_id, parent_id);
        self.nodes.push(node);
        self.children.push(HashSet::new());
        self.children[parent_id].insert(child_id);
        self.node_count += 1;
    }
    
    pub fn get_children(&self, parent_id: usize) -> &HashSet<usize>{
        &self.children[parent_id]
    }

    pub fn get_parent(&self, child_id:usize) -> Option<usize>{
        for (i, set) in self.children.iter().enumerate(){
            //I KNOW THIS IS INNEFICIENT AAAAAAAAAAAAAA
            if set.contains(&child_id){
                return Some::<usize>(i);
            }
        }
        None
    }
    
    pub fn get_data(&self, node_id:usize) -> &T{
        &self.nodes[node_id]
    }
    
    pub fn get_mut_data(&mut self, node_id:usize) -> &mut T{
        &mut self.nodes[node_id]
    }

    pub fn get_iter(&self) ->std::slice::Iter<T> {
        self.nodes.iter()
    }

    pub fn pretty_print(&self){
        println!("{:?}", self.get_data(0));
        for child in self.get_children(0){
            self.pretty_print_aux(*child, 0);
        }
    }

    fn pretty_print_aux(&self, node_index: usize, depth:u32){
        for _ in 0..depth{
            print!(" ");
        }
        println!("|{:?}", self.get_data(node_index));
        for child in self.get_children(node_index){
            self.pretty_print_aux(*child, depth+1);
        }
    }
}

