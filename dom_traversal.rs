
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: String,
  pub children: Vec<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  pub fn new(val: String) ->Self { 
    TreeNode {
      val,
      children: vec![],
    }
  }
}

fn traverse(root: Option<Rc<RefCell<TreeNode>>>, result:&mut Vec<Vec<String>>) {
    match &root {
      Some(node) => {
        let mut queue: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
        queue.push_front(node.clone());
          while !queue.is_empty() {
            let level_size = queue.len();
            let mut current_level: Vec<String> = Vec::new();
            for _i in 0..level_size {
              let current_node = queue.pop_back().unwrap();
              current_level.push(current_node.borrow().val.to_string());
              for child in current_node.borrow().children.clone().into_iter(){
                  queue.push_front(child);
              }
            }
            result.push(current_level);
          }
        },
        None => {return }
      };
      
      
}



fn main() {
    let mut root = TreeNode::new("body".to_string());
    root.children.push(Rc::new(RefCell::new(TreeNode::new("div".to_string()))));
    root.children.push(Rc::new(RefCell::new(TreeNode::new("h1".to_string()))));
    root.children.push(Rc::new(RefCell::new(TreeNode::new("div".to_string()))));
    root.children[0].borrow_mut().children.push(Rc::new(RefCell::new(TreeNode::new("h2".to_string()))));
    root.children[0].borrow_mut().children[0].borrow_mut().children.push(Rc::new(RefCell::new(TreeNode::new("ul".to_string()))));
    root.children[0].borrow_mut().children.push(Rc::new(RefCell::new(TreeNode::new("h3".to_string()))));
    root.children[0].borrow_mut().children[1].borrow_mut().children.push(Rc::new(RefCell::new(TreeNode::new("a".to_string()))));
    root.children[0].borrow_mut().children[1].borrow_mut().children.push(Rc::new(RefCell::new(TreeNode::new("p".to_string()))));
    root.children[0].borrow_mut().children[1].borrow_mut().children.push(Rc::new(RefCell::new(TreeNode::new("table".to_string()))));
    root.children[2].borrow_mut().children.push(Rc::new(RefCell::new(TreeNode::new("p".to_string()))));
    root.children[2].borrow_mut().children.push(Rc::new(RefCell::new(TreeNode::new("a".to_string()))));
    root.children[2].borrow_mut().children.push(Rc::new(RefCell::new(TreeNode::new("p".to_string()))));

      let mut result: Vec<Vec<String>> = Vec::new();

     traverse(Some(Rc::new(RefCell::new(root))),&mut result);
    println!("{:?}",result );
}
