use std::time::Duration;
use std::thread;

fn main() {
    let apple = Rc::new("the same apple");

    for _ in 0..10 {
        let apple = Rc::clone(&apple);

        thread::spawn(move || {
            println!("{:?}", apple);
        });
    }

    thread::sleep(Duration::from_secs(1));
}