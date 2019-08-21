use std::sync::{Mutex,Arc};
use std::thread;

fn main() {
    let m = Arc::new(Mutex::new(10));
    let mut threads =  vec![];
    for _  in 0 .. 5 {
      let counter = Arc::clone(&m);
      let c = thread::spawn(move || {
        {
            *counter.lock().unwrap() += 1;
        }
        let updated = *counter.lock().unwrap();
        updated
      });
      threads.push(c);
    } 
    
    for th in threads {
     let updated = th.join().unwrap();
     println!("{:?}", updated);
    } 
    println!("Final: {}", *m.lock().unwrap());
}
