//use std::process::exit;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

#[derive(Debug, Clone)]
struct MyStruct {
    x: i32,
}

pub(crate) fn main() -> Result<(), Box<dyn std::error::Error>> {
    let x = MyStruct { x: 5 };

    // create a scope
    thread::scope(|scope| {
        // move is not required since we are in a scope
        // and MyStruct implements Sync
        scope.spawn(|| {
            for i in 0..10 {
                println!("{i} Hello from thread 1: {:?}", x);
                thread::sleep(Duration::from_millis(1));
            }
        });

        // move is not required since we are in a scope
        scope.spawn(|| {
            for i in 0..10 {
                println!("{i} Hello from thread 2: {:?}", x);
                thread::sleep(Duration::from_millis(1));
            }
        });

        //exit(0);
        //return ();

        // main thread can also access x, no clone required
        for i in 0..10 {
            println!("{i} Hello from main thread {:?}", x);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // Join is automatic

    // x wasn't moved into the scope
    println!("{:?}", x);

    Ok(())
}

pub(crate) fn mpsc() -> Result<(), Box<dyn std::error::Error>> {
    thread::scope(|scope| {
        // create a channel
        // tx and rx do NOT implement Sync, so cannot be shared by reference
        let (tx, rx) = mpsc::channel::<i32>();
        let tx1 = tx.clone();
        let tx2 = tx.clone();
        let tx3 = tx.clone();

        // spawn a producer thread
        // tx1 must be moved because it doesn't implement Sync so can't be borrowed
        scope.spawn(move || {
            thread::sleep(Duration::from_secs(5));
            tx1.send(50).unwrap();
            println!("TX 1 completed: 50");
        });

        // spawn a second producer thread
        scope.spawn(move || {
            thread::sleep(Duration::from_secs(2));
            tx2.send(123).unwrap();
            println!("TX 2 completed: 123");
        });

        // spawn a third producer thread
        scope.spawn(move || {
            thread::sleep(Duration::from_secs(10));
            tx3.send(66).unwrap();
            println!("TX 3 completed: 66");
        });

        // rx must be moved into the thread as it cannot be shared (not Sync)
        scope.spawn(move || {
            let result = rx.recv().unwrap() + rx.recv().unwrap() + rx.recv().unwrap();
            println!("RX completed: {result}");
        });
    });

    println!("Complete");

    Ok(())
}
