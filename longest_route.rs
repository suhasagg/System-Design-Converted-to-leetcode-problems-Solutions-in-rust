fn longest_route(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {

     if let Some(node) = root {
        let node = node.borrow();
        let left = height(node.left.clone());
        let right = height(node.right.clone());
        let l_path = longest_route(node.left.clone());
        let r_path = longest_route(node.right.clone());
        let res = cmp::max(left + right + 1, cmp::max(l_path, r_path));

        return res;
    } else {
        0
    }
}

fn height(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if let Some(node) = root {
        let node = node.borrow();
        let left = height(node.left.clone());
        let right = height(node.right.clone());
        return cmp::max(left,right)+1;
    } else {
        0
    }
}

fn main() {
    let mut root = TreeNode::new("a".to_string());
    root.left = Some(Rc::new(RefCell::new(TreeNode::new("b".to_string()))));
    root.right = Some(Rc::new(RefCell::new(TreeNode::new("c".to_string()))));
    root.left.clone().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new("d".to_string()))));
    root.right.clone().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new("e".to_string()))));
    root.right.clone().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new("f".to_string()))));
  println!("{}{:}{}","The longest route will pass through " ,longest_route(Some(Rc::new(RefCell::new(root)))) , " checkpoints");

}
