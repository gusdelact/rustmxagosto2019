use std::sync::Mutex;
use std::thread;

fn main() {
    let m = Mutex::new(10);
    let mut threads =  vec![];
    for _  in 0 .. 5 {
      let c = thread::spawn(move || {
        {
            *m.lock().unwrap() += 1;
        }
        let updated = *m.lock().unwrap();
        updated
      });
      threads.push(c);
    } 
    
    for th in threads {
     let updated = th.join().unwrap();
     println!("{:?}", updated);
    } 
}
