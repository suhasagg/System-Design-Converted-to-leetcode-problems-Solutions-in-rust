use std::collections::LinkedList;

fn merge2_countries(l1:&mut LinkedList<i32>, l2:&mut LinkedList<i32>)->LinkedList<i32> {
    let mut prev: LinkedList<i32> = LinkedList::new();
    while !l1.is_empty() && !l2.is_empty() {
        if l1.front() <= l2.front() {
            prev.push_back(l1.pop_front().unwrap());
        } else {
            prev.push_back(l2.pop_front().unwrap());
        }
    } 
    if l1.is_empty()
     { prev.append(l2);}
    else
      { prev.append(l1);}

    return prev;
}

fn merge_kcountries(lists:&mut Vec<LinkedList<i32>>)->LinkedList<i32> {

  if lists.len() > 0{
    let mut res: LinkedList<i32> = lists[0].clone();

    for i in 1..lists.len(){
      res = merge2_countries(&mut res,&mut lists[i]);
    }

    return res;
  }
  let mut l: LinkedList<i32> = LinkedList::new();
  l.push_back(-1);
  return l;
}

fn main() {
    let mut a: LinkedList<i32> = LinkedList::new();

    a.push_back(11);
    a.push_back(41);
    a.push_back(51);

    let mut b: LinkedList<i32> = LinkedList::new();
    b.push_back(21);
    b.push_back(23);
    b.push_back(42);

    let mut c: LinkedList<i32> = LinkedList::new();
    c.push_back(25);
    c.push_back(56);
    c.push_back(66);
    c.push_back(72);

    let mut list1: Vec<LinkedList<i32>> = Vec::new();
    list1.push(a);
    list1.push(b);
    list1.push(c);

    println!("{:?}",merge_kcountries(&mut list1));
}
