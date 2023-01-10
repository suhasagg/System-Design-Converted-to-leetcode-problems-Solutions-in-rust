use std::rc::Rc;
use std::cell::RefCell;
#[derive(Default, Debug, PartialEq, Eq, Clone)]
pub struct Node {
  pub start: i32,
  pub end: i32,
  pub left_child: Option<Rc<RefCell<Node>>>,
  pub right_child: Option<Rc<RefCell<Node>>>
}

impl Node {
  pub fn new(start: i32, end: i32) ->Self { 
    Node {
      start: start,
      end: end,
      left_child: None,
      right_child: None
    }
  }
}
#[derive(Default,Debug, PartialEq, Eq, Clone)]
pub struct BST{
  pub root: Node
}

impl BST {
  pub fn new(start: i32, end: i32) ->Self { 
   BST{
     root: Node::new(start,end)
   }
  }

  pub fn insert(&mut self, start: i32, end: i32)->bool {
      if let Some(_node) = Some(self.root.clone()){
        let new_node = Some(Rc::new(RefCell::new(Node::new(start,end))));
        return Self::add_node(&mut self.root, new_node);
      } else {
          self.root = Node::new(start,end);
          return true;
      }
  }

  pub fn add_node(current_node:&mut Node, new_node: Option<Rc<RefCell<Node>>>)->bool {
    
    let new = new_node.clone().unwrap().borrow().clone();
      if new.start >= current_node.end{
          if current_node.right_child.is_none(){
              current_node.right_child = new_node.clone();
              return true;
          }
          return Self::add_node(&mut current_node.right_child.as_ref().unwrap().borrow_mut(), new_node);
      }
      else if new.end <= current_node.start{
          if current_node.left_child.is_none(){
              current_node.left_child = new_node.clone();
              return true;
          }
          return Self::add_node(&mut current_node.left_child.as_ref().unwrap().borrow_mut(), new_node);
      }
      else{
          return false;
      }
  }
   
}


fn check_meeting(meeting_times: Vec<Vec<i32>>,  new_meeting: Vec<i32>) ->bool{

    let mut tree = BST::new(1,3);
    //BST tree;
    for meeting in meeting_times.into_iter(){
        tree.insert(meeting[0], meeting[1]);
    }
    return tree.insert(new_meeting[0], new_meeting[1]);
}

// Driver Code
fn main() {
  // First set of meetings
  let meeting_times1: Vec<Vec<i32>> = vec![ vec![4, 6], vec![8, 10], vec![10, 12], vec![13, 15]];
  let new_meeting: Vec<i32>= vec![7, 8];
  println!("{:?}",check_meeting( meeting_times1.to_vec(), new_meeting));
    // Second set of meetings
  let new_meeting2: Vec<i32>= vec![9, 11];
  println!("{:?}",check_meeting( meeting_times1.to_vec(), new_meeting2));
  
}
