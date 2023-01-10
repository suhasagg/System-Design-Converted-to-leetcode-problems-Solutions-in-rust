use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
#[derive(Debug)]
struct Node {
    key: i32,
    value: i32,
    freq: i32,
    prev: Option<Rc<RefCell<Node>>>,
    next: Option<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(key: i32, value: i32) -> Self {
        let freq = 1;
        let prev = None;
        let next = None;
        Node {
            key,
            value,
            freq,
            prev,
            next,
        }
    }
}

#[derive(Default, Debug)]
struct LinkedList {
    head: Option<Rc<RefCell<Node>>>,
    tail: Option<Rc<RefCell<Node>>>,
}

impl LinkedList {
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

    
    fn pop_front(&mut self) -> Option<Rc<RefCell<Node>>> {
        if let Some(head) = self.head.as_ref().map(|a| a.clone()) {
            if let Some(head_next) = head.borrow_mut().next.take() {
                head_next.borrow_mut().prev = None;
                self.head = Some(head_next);
            } else {
                self.tail = None;
                self.head = None;
            }
            return Some(head);
        }
        None
    }
    
    fn is_empty(&self) -> bool {
        self.head.is_none() && self.tail.is_none()
    }
}

struct LfuStructure {
    capacity: i32,
    count: i32,
    min_freq: RefCell<i32>,
    key_dict: HashMap<i32, Rc<RefCell<Node>>>,
    freq_dict: RefCell<HashMap<i32, LinkedList>>,
}

impl LfuStructure {
    fn new(capacity: i32) -> Self {
        let capacity = capacity;
        let count = 0;
        let min_freq = RefCell::new(0);
        let key_dict = HashMap::new();
        let freq_dict = RefCell::new(HashMap::new());
        LfuStructure {
            capacity,
            count,
            min_freq,
            key_dict,
            freq_dict,
        }
    }

    fn min_freq(&self) -> i32 {
        *self.min_freq.borrow()
    }

    fn set_min_freq(&self, freq: i32) {
        *self.min_freq.borrow_mut() = freq;
    }

    fn get(&self, key: i32) -> i32 {
        if self.capacity == 0 {
            return -1;
        }
        if let Some(node) = self.key_dict.get(&key) {
            let value = node.borrow().value;
            self.update_freq(node.clone());
            value
        } else {
            -1
        }
    }

    fn set(&mut self, key: i32, val: i32) {
        if self.capacity == 0 {
            return;
        }
        if let Some(node) = self.key_dict.get(&key) {
            node.borrow_mut().value = val;
            self.update_freq(node.clone());
        } else {
            self.count += 1;
            
            let node = Rc::new(RefCell::new(Node::new(key, val)));
            self.key_dict.insert(key, node.clone());
            self.freq_dict
                .borrow_mut()
                .entry(1)
                .or_default()
                .push_back(&node);
            self.set_min_freq(1);
            if self.count > self.capacity {
                let node = self.pop_front_node(self.min_freq()).unwrap();
                self.key_dict.remove(&node.borrow().key);
            }
        }
    }

    fn update_freq(&self, node: Rc<RefCell<Node>>) {
        let freq = node.borrow().freq;
        node.borrow_mut().freq += 1;
        self.push_back_node(freq + 1, self.take_node(freq, node));
        if freq == self.min_freq() && self.freq_dict.borrow_mut().entry(freq).or_default().is_empty() {
            self.set_min_freq(freq + 1);
        }
    }

    fn take_node(&self, freq: i32, node: Rc<RefCell<Node>>) -> Rc<RefCell<Node>> {
        let mut freq_dict = self.freq_dict.borrow_mut();
        let linked_list = freq_dict.get_mut(&freq).unwrap();
        {
            let mut node = node.borrow_mut();
            match (node.prev.take(), node.next.take()) {
                (Some(prev), Some(next)) => {
                    next.borrow_mut().prev = Some(prev.clone());
                    prev.borrow_mut().next = Some(next);
                }
                (None, Some(next)) => {
                    next.borrow_mut().prev = None;
                    linked_list.head = Some(next);
                }
                (Some(prev), None) => {
                    prev.borrow_mut().next = None;
                    linked_list.tail = Some(prev);
                }
                (None, None) => {
                    linked_list.head = None;
                    linked_list.tail = None;
                }
            }
        }
        node
    }

    fn push_back_node(&self, freq: i32, node: Rc<RefCell<Node>>) {
        let mut freq_dict = self.freq_dict.borrow_mut();
        let linked_list = freq_dict.entry(freq).or_default();
        linked_list.push_back(&node);
    }

    fn pop_front_node(&self, freq: i32) -> Option<Rc<RefCell<Node>>> {
        if let Some(linked_list) = self.freq_dict.borrow_mut().get_mut(&freq) {
            linked_list.pop_front()
        } else {
            None
        }
    }
    fn print(&self) {
        for (key, value) in self.key_dict.iter() {
            print!("({}, {})", key, value.borrow().value);
        }
        println!("");
    }
}

fn main() {
  let mut obj = LfuStructure::new(2);
    println!("The most frequently watched titles are: (key, value)");
    
    obj.set(1, 1);
    obj.set(2, 2);
    obj.print();
    
    obj.get(1);

    obj.set(3, 3);
    obj.print();

    obj.get(2);
    obj.set(4, 4);
    obj.print();

    obj.get(1);
    obj.get(3);
    obj.get(4);
    obj.print();
}
