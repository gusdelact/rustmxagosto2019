use std::thread;
use std::time::Duration;

fn main() {
    let v = vec![10, 20, 30];

    let handle = thread::spawn(move || {
        for i in 0 .. 3 {
            println!("Here's a vector: {:?}", v[i]);
        }

    });

    
    for i in 1..5   {
         println!("Hola {} del hilo principal", i);
        thread::sleep(Duration::from_millis(1));
    }
    handle.join().unwrap();
}
