use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
#[derive(Debug)]
struct Node {
    key: i32,
    value: i32,
    prev: Option<Rc<RefCell<Node>>>,
    next: Option<Rc<RefCell<Node>>>,
}

#[derive(Default, Debug)]
struct LinkedList {
    head: Option<Rc<RefCell<Node>>>,
    tail: Option<Rc<RefCell<Node>>>,
}

impl LinkedList {
    fn new() -> LinkedList {
        Default::default()
    }

    fn move_to_tail(&mut self, node: &Rc<RefCell<Node>>) {
        let prev = node.borrow().prev.as_ref().map(|a| Rc::clone(a));
        let next = node.borrow().next.as_ref().map(|a| Rc::clone(a));
        match (&prev, &next) {
            (None, None) => {
                // this is the only node in list, so do nothng
            }
            (Some(_), None) => {
                // this node is already the tail, so do nothing
            }
            (None, Some(next)) => {
                // this node is at the head, so we need to move it to the tail
                node.borrow_mut().next = None;
                node.borrow_mut().prev = None;
                next.borrow_mut().prev = None;
                self.head = Some(Rc::clone(next));

                let prev_tail = self.tail.as_ref().unwrap();
                prev_tail.borrow_mut().next = Some(Rc::clone(node));
                node.borrow_mut().prev = Some(Rc::clone(prev_tail));
                self.tail = Some(Rc::clone(node));
            }
            (Some(prev), Some(next)) => {
                node.borrow_mut().next = None;
                node.borrow_mut().prev = None;

                prev.borrow_mut().next = Some(Rc::clone(next));
                next.borrow_mut().prev = Some(Rc::clone(prev));

                let prev_tail = self.tail.as_ref().unwrap();
                prev_tail.borrow_mut().next = Some(Rc::clone(node));
                node.borrow_mut().prev = Some(Rc::clone(prev_tail));
                self.tail = Some(Rc::clone(node));
            }
        }
    }

    fn push_back(&mut self, node: &Rc<RefCell<Node>>) {
        match &self.tail {
            None => {
                self.head = Some(Rc::clone(node));
                self.tail = Some(Rc::clone(node));
            }
            Some(prev_tail) => {
                Rc::clone(node).borrow_mut().prev = Some(Rc::clone(prev_tail));
                prev_tail.borrow_mut().next = Some(Rc::clone(node));
                self.tail = Some(Rc::clone(node));
            }
        }
    }

    fn remove_head(&mut self) -> Option<Rc<RefCell<Node>>> {
        if let Some(head) = self.head.as_ref().map(|a| a.clone()) {
            if let Some(head_next) = head.borrow().next.as_ref() {
                head_next.borrow_mut().prev = None;
                self.head = Some(Rc::clone(head_next));
            }
            return Some(head);
        }
        None
    }
}
#[derive(Debug)]
struct LRUCache {
    map: HashMap<i32, Rc<RefCell<Node>>>,
    cache_vals: LinkedList,
    size: i32,
    capacity: i32,
}

impl LRUCache {
    fn new(capacity: i32) -> Self {
        Self {
            map: HashMap::new(),
            cache_vals: LinkedList::new(),
            size: 0,
            capacity,
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        if let Some(node) = self.map.get(&key) {
            self.cache_vals.move_to_tail(node);
            return node.borrow().value;
        } else {
            return -1;
        }
    }

    fn set(&mut self, key: i32, value: i32) {
        if let Some(node) = self.map.get(&key) {
            self.cache_vals.move_to_tail(node);
            node.borrow_mut().value = value;
        } else {
            // remove tail node if needed
            if self.size >= self.capacity {
                if let Some(prev_head) = self.cache_vals.remove_head() {
                    self.map.remove(&prev_head.borrow().key);
                };
            }
            // add node to list head
            let node = Rc::new(RefCell::new(Node {
                prev: None,
                next: None,
                key,
                value,
            }));
            self.cache_vals.push_back(&node);
            // update hashmap
            self.map.insert(key, node);
            // update size
            self.size += 1;
        }
    }
    fn print(&mut self) {
        let mut head = self.cache_vals.head.as_ref().map(|a| a.clone());
        while !head.is_none() {
            let temp = head.map(|a| a.clone()).unwrap();
            print!("({}, {})", temp.borrow().key, temp.borrow().value);
            head = temp.borrow().next.clone();
        }
        println!("");
    }
}
fn main(){
    let mut obj = LRUCache::new(3);
    println!("The most recently watched titles are: (key, value)");
    
    obj.set(10, 20);
    obj.print();
    
    obj.set(15, 25);
    obj.print();

    obj.set(20, 30);
    obj.print();

    obj.set(25, 35);
    obj.print();

    obj.set(5, 40);
    obj.print();

    obj.get(25);
    obj.print();
}
