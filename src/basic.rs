use std::thread;
use std::time::Duration;

#[derive(Debug, Clone)]
struct MyStruct {
    x: i32,
}

pub(crate) fn main() -> Result<(), Box<dyn std::error::Error>> {
    let x = MyStruct { x: 5 };

    // leak 'x' to make it static from this point onwards (doesn't work)
    // let x = Box::from(x);
    // let x = Box::leak(x);

    // If we want main thread to access x, we need to clone it
    let x1 = x.clone();

    // Closure must take ownership since it can outlive current function
    let thread_handle = thread::spawn(move || {
        for i in 0..10 {
            println!("{i} Hello from a thread {:?}", &x1);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 0..10 {
        println!("{i} Hello from main thread {:?}", x);
        thread::sleep(Duration::from_millis(1));
    }

    let _ = thread_handle.join();

    // x1 was moved into the scope so no longer usable here
    //println!("{:?}", x1);  // error

    Ok(())
}
