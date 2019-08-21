use std::thread;
use std::io;
use std::io::prelude::*;

fn pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    // We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
    write!(stdout, "Press any key to continue...").unwrap();
    stdout.flush().unwrap();

    // Read a single byte and discard
    let _ = stdin.read(&mut [0u8]).unwrap();
}

fn main() {
    let mut shared:u32 = 0;
    for i in 0 .. 5 {
      thread::spawn(move  || {
            println!("thread {}",i );
            shared = shared + i;
            let p_shared = &shared;
            println!("shared value {} p_shared value {:p}",shared, p_shared  );
      });
    }

  pause();
  println!("shared value {}",shared );

}
