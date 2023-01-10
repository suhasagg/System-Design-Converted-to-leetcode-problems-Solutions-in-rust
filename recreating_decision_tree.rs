use std::collections::HashMap;
fn create_tree_from_val( preorder: Vec<String>, left: i32, right: i32, inorder_val_idx: HashMap<String,i32>, mut preorder_idx:&mut i32)-> Option<Rc<RefCell<TreeNode>>> {
    // if there are no elements to construct the tree
    if left > right { 
        return None;
    }

    // select the pIdx element as the root and increment it
    let mut rootVal: String = String::new(); 
    rootVal = preorder[*preorder_idx as usize].to_string();
    let mut root: TreeNode = TreeNode::new(rootVal.to_string());
    
    *preorder_idx+=1;

    // Constructing the left and right subtree using the root value i.e., rootVal 
    // from the inorder_val_idx Dictionary
    root.left = create_tree_from_val(preorder.to_vec(), left, inorder_val_idx.clone().get(&rootVal).unwrap() - 1, inorder_val_idx.clone(), &mut preorder_idx);
    root.right = create_tree_from_val(preorder.to_vec(), inorder_val_idx.clone().get(&rootVal).unwrap() + 1, right, inorder_val_idx.clone(), &mut preorder_idx);

    return Some(Rc::new(RefCell::new(root)));
}

fn re_creating_decision_tree(preorder: Vec<String>, inorder: Vec<String>,mut preorder_idx: &mut i32)-> Option<Rc<RefCell<TreeNode>>> {

    // build a Dictionary to store value -> its index relations
    let mut inorder_val_idx: HashMap<String, i32> = HashMap::new();
    for i in 0..inorder.len() {
      inorder_val_idx.entry(inorder[i].to_string()).or_insert(i as i32);
    }

    return create_tree_from_val(preorder.to_vec(), 0, preorder.len() as i32 - 1, inorder_val_idx, &mut preorder_idx);
}

fn print_tree(root: Option<Rc<RefCell<TreeNode>>>,mut space:&mut i32){
      const COUNT: i32 = 10;
      // Base case
      if root == None
          {return;}
  
      // Increase distance between levels
      // Process right child first
      print_tree(root.clone().unwrap().borrow().right.clone(),&mut (*space + COUNT));
  
      // Print current node after space
      for i in COUNT..*space + COUNT{
          print!(" ");
      }
     print!("{}{:?}","-" , root.clone().unwrap().borrow().val );
     println!();
      
      // Process left child
      print_tree(root.unwrap().borrow().left.clone(),&mut (*space + COUNT));
  }


fn main() {
    // Driver code
    let preorder: Vec<String> = vec!["subject", "viewed", "notviewed", "similar", "nonsimilar"].into_iter().map(String::from).collect();
    let inorder: Vec<String> = vec!["viewed", "subject", "similar", "notviewed", "nonsimilar"].into_iter().map(String::from).collect();
    let mut preorder_idx: i32 = 0;
    let result = re_creating_decision_tree(preorder, inorder, &mut preorder_idx);  
     print_tree(result,&mut 0);
}
