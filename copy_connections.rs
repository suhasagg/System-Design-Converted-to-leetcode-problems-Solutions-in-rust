use std::collections::HashSet;
use std::collections::HashMap;
use rand::prelude::*;
use std::rc::Rc;
use std::cell::RefCell;
#[derive(Debug, PartialEq, Eq)]
pub struct Node {
  pub data: i32,
  pub friends: Vec<Rc<RefCell<Node>>>,
}

impl Node {
  pub fn new(data: i32) ->Self { 
    Node {
      data,
      friends: vec![],
    }
  }
}


#[derive(Debug, PartialEq, Eq)]
pub struct CloneGraph {
  
}

  fn shuffle(edges: &mut Vec<(i32,i32)>) {
    let mut rng = thread_rng();
      let n = edges.len();
      for i in 0..n {
          let j = rng.gen_range(i..n) ;
          edges.swap(i, j);
      }
  }

impl CloneGraph {

  pub fn create_test_graph_directed(nodes_count: i32, edges_count: i32)->Vec<Rc<RefCell<Node>>> {
            let mut vertices: Vec<Rc<RefCell<Node>>> = Vec::new();
            for i in 0..nodes_count {
                vertices.push(Rc::new(RefCell::new(Node::new(i))));
                
            }
            let mut all_edges: Vec<(i32,i32)> = Vec::new();
            for i in 0..vertices.len() {
                for  j in i + 1..vertices.len() {
                    all_edges.push((i as i32, j as i32));
                }
            }
            shuffle(&mut all_edges);
            let mut i = 0;
            while i < edges_count && i < all_edges.len() as i32{
                let edge = all_edges[i as usize];
                vertices[edge.0 as usize].borrow_mut().friends.push(vertices.get(edge.1 as usize).unwrap().clone());
                vertices[edge.1 as usize].borrow_mut().friends.push(vertices.get(edge.0 as usize).unwrap().clone());
                i = i+1;
            }
           
            return vertices;
        }

      pub fn print_graph(vertices: Vec<Rc<RefCell<Node>>>) {
            for n in vertices.into_iter() {
                println!("{:}{}",n.clone().borrow().data ,": {");
                for t in n.clone().borrow_mut().friends.to_vec().into_iter() {
                    println!("{:}{}",t.clone().borrow().data , " ");
                }
            }
        }

        pub fn print_graph_two(root: &Option<Rc<RefCell<Node>>>, visited_nodes: &mut HashSet<i32>) {
            if root.is_none() || visited_nodes.contains(&root.as_ref().unwrap().clone().borrow().data) {
                return;
            }

            visited_nodes.insert(root.as_ref().unwrap().clone().borrow().data);

            print!("{:}{}",root.as_ref().unwrap().clone().borrow().data , ": {");
            for n in root.as_ref().unwrap().clone().borrow().friends.to_vec().into_iter() {
                print!("{:}{}", n.clone().borrow().data, " ");
            }
            println!("{}","}");

            for n in root.as_ref().unwrap().clone().borrow().friends.to_vec().into_iter() {
                Self::print_graph_two(&Some(n), visited_nodes);
            }
        }

        pub fn print_graph_one(root: &Option<Rc<RefCell<Node>>>) {
          let mut visited_nodes: HashSet<i32> = HashSet::new();
          Self::print_graph_two(&root, &mut visited_nodes);
        }

}


fn clone_rec(root: &mut Option<Rc<RefCell<Node>>>, nodes_completed: &mut HashMap<i32, Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Node>>> {
    let data = root.as_ref().unwrap().clone().borrow().data;
    let new_node = Rc::new(RefCell::new(Node::new(data)));
    nodes_completed.insert(data, new_node.clone());

    for p in root.as_ref().unwrap().clone().borrow().friends.to_vec().into_iter() {
      let data = p.clone().borrow().data;
      if !nodes_completed.contains_key(&data) {
          let cloned = clone_rec(&mut Some(p.clone()), nodes_completed);
          new_node.borrow_mut().friends.push(cloned.unwrap().clone());
      } else {
          new_node.borrow_mut().friends.push(nodes_completed.get(&data).unwrap().clone());
      }
    }
    return Some(new_node);
}

fn clone(root:  &mut Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Node>>>{
    let mut nodes_completed: HashMap<i32, Rc<RefCell<Node>>> = HashMap::new();  
    return clone_rec(root, &mut nodes_completed);
}


fn main() {
  let vertic = CloneGraph::create_test_graph_directed(7, 18);
  CloneGraph::print_graph_one(&Some(vertic[0].clone()));
  let cp = clone(&mut Some(vertic[0].clone()));

  println!("After copy");
  CloneGraph::print_graph_one(&cp);

} 
