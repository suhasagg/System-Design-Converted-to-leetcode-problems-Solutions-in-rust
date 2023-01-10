use std::collections::HashMap;
struct RequestLimiter {
    requests: HashMap<String, i32>,    
    
}

impl RequestLimiter {
    fn new() -> Self {
        RequestLimiter {
            requests: HashMap::new(),
        }
    }
    // Returns true if the request should be printed in the given timestamp, 
  //otherwise returns false.
    fn request_approver(&mut self, timestamp: i32, request: String) -> bool{
        //println!("{}",&request);
        if self.requests.get(&request) == None {
            *self.requests.entry(request).or_insert(0) += timestamp;
            print!("{}","Request Accepted\n");
            return true;
        }

        let old_timestamp = self.requests[&request];

        if timestamp - old_timestamp >= 5 {
          *self.requests.get_mut(&request).unwrap() = timestamp;
          print!("{}","Request Accepted\n");
          return true;
        } 
        else
        {
          print!("{}","Request Rejected\n");
          return false;
        }
    }

}


fn main(){
    // Driver code  
    let mut obj = RequestLimiter::new();

    obj.request_approver(1, "send_message".to_string());
    obj.request_approver(2, "block".to_string());
    obj.request_approver(3, "send_message".to_string());
    obj.request_approver(8, "block".to_string());
    obj.request_approver(10, "send_message".to_string());
    obj.request_approver(11, "send_message".to_string());
}
