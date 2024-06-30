use std::error::Error;
use std::thread;

pub(crate) fn main() -> Result<(), Box<dyn Error>> {
    let items = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let result = process_fork_join(items)?;
    println!("final result: {result}");

    Ok(())
}

pub(crate) fn process_fork_join(items: Vec<i32>) -> Result<i64, Box<dyn Error>> {
    const N_THREADS: usize = 4;

    let worklists = split_vec_into_chunks(items, N_THREADS);

    let mut thread_handles = vec![];

    for worklist in worklists {
        thread_handles.push(thread::spawn(move || process_items(&worklist)));
    }

    let mut sum: i64 = 0;
    for handle in thread_handles {
        let x = handle.join();
        match x {
            Ok(x) => {
                println!("Thread finished with result: {}", x);
                sum += x as i64;
            }
            Err(e) => println!("Thread panicked: {:?}", e),
        }
    }

    Ok(sum)
}

fn process_items(items: &[i32]) -> i32 {
    items.iter().sum()
}

fn split_vec_into_chunks(items: Vec<i32>, num_threads: usize) -> Vec<Vec<i32>> {
    assert_ne!(num_threads, 0);

    let chunk_size = (items.len() + num_threads - 1) / num_threads; // Ceiling division
    items
        .chunks(chunk_size)
        .map(|chunk| chunk.to_vec())
        .collect()
}
