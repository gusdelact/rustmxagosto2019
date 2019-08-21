use std::thread;
fn main() {
   let mut handles = vec![];

   for _ in 0..10 {
       let handle = thread::spawn( || {
             println!("Thread creado");
       });
       handles.push(handle);
   }

   for th in handles {
       th.join().unwrap();
   }
}
