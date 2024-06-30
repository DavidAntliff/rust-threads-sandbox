use std::sync::Arc;
use std::thread;
use std::time::Duration;

#[derive(Debug, Clone)]
struct MyStruct {
    x: i32,
}

pub(crate) fn main() -> Result<(), Box<dyn std::error::Error>> {
    let d = MyStruct { x: 5 };

    // Move the data to the heap, and wrap with an Arc
    let x = Arc::new(d);

    // The thread takes ownership (via move closure)
    // but only the Arc is cloned, not the actual data
    let x1 = x.clone();

    // Closure must take ownership since it can outlive current function
    let thread_handle = thread::spawn(move || {
        for i in 0..10 {
            println!("{i} Hello from a thread {:?}", &x1);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 0..10 {
        println!("{i} Hello from main thread {:?}", &x);
        thread::sleep(Duration::from_millis(1));
    }

    let _ = thread_handle.join();

    // x1 was moved into the thread so no longer usable here
    //println!("{:?}", x1);  // error

    Ok(())
}
