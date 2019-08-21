use std::thread;
use std::time::Duration;

fn main() {
  thread::spawn(|| {
      for i in 1..10 {
          println!("Hilo # {} !", i);
          thread::sleep(Duration::from_millis(1));
      }
   }
  );

  for i in 1..10 {
       println!("Hola {} del hilo principal", i);
      thread::sleep(Duration::from_millis(1));
  }
}
