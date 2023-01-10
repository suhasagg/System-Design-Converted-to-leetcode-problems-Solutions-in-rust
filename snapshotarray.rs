use std::collections::HashMap;
#[derive(Debug)]
pub struct Snapshot {
  pub snap_id: i32,
  pub ncount: i32,
  pub node_state: HashMap<i32,HashMap<i32,i32>>,
}

impl Snapshot {
  pub fn new(length: i32) ->Self { 
    
    let mut init: HashMap<i32,HashMap<i32,i32>> = HashMap::new();
      for x in 0..length {
        let mut n: HashMap<i32,i32> = HashMap::new();
        n.entry(x).or_insert(0);
        init.entry(0).or_insert(n);
      }
    Snapshot {
      snap_id : 0,
      ncount: length,
      node_state: init
    }
  }

  pub fn set_state(&mut self,idx: i32, val: i32){
    if idx < self.ncount{
      *self.node_state.get_mut(&self.snap_id).unwrap().get_mut(&idx).unwrap() = val;
    }
  }

  pub fn snap(&mut self)-> i32{
      self.snap_id += 1;
      // Deep copying node_state[snap_id -1] to node_state[snap_id]
      let s = self.node_state.get(&(self.snap_id -1)).cloned().unwrap();
      if self.node_state.contains_key(&self.snap_id)
      {
        *self.node_state.get_mut(&self.snap_id).unwrap() = s.clone();
      }
      else{
        self.node_state.entry(self.snap_id).or_insert(s.clone());
      }
      return self.snap_id -1;
  }

  pub fn fetch_state(&mut self,idx: i32, snap_id: i32)->Option<&i32>{
    if snap_id < self.snap_id && snap_id >= 0{
            if self.node_state.get(&snap_id).unwrap().contains_key(&idx){
                return self.node_state.get(&snap_id).unwrap().get(&idx);
            } 
            else 
                {return Some(&0);}
        }
        return None;
  }

}


    
fn main(){
    let mut snapshot_arr: Snapshot = Snapshot::new(3);
     snapshot_arr.set_state(0,5);  
     snapshot_arr.snap();
     snapshot_arr.set_state(0,1);
     snapshot_arr.snap();
     let mut output = snapshot_arr.fetch_state(0,0);
     if output.is_none(){
       println!("null");
     }
     else{
       println!("{:}",output.unwrap());
     }
    
    
     output = snapshot_arr.fetch_state(0,4);
    if output.is_none(){
       println!("null");
     }
     else{
       println!("{:}",output.unwrap());
     }
    
     output = snapshot_arr.fetch_state(5,0);
    if output.is_none(){
       println!("null");
     }
     else{
       println!("{:}",output.unwrap());
     }

    output = snapshot_arr.fetch_state(0,1);
    if output.is_none(){
       println!("null");
     }
     else{
       println!("{:}",output.unwrap());
     }

     output = snapshot_arr.fetch_state(0,2);
    if output.is_none(){
       println!("null");
     }
     else{
       println!("{:}",output.unwrap());
     }
}
