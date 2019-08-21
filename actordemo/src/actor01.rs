extern crate actix;
use actix::{Actor, Addr, Arbiter, Context, System};

struct MyActor;

impl Actor for MyActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
       println!("I am alive!");
       System::current().stop(); // <- stop system
    }
}

fn main() {
    let system = System::new("test");

    let addr = MyActor.start();

    system.run();
}
