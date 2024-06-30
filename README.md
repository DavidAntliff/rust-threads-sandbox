# Thread Sandbox

## Examples

- [Basic](src/basic.rs) - basic non-scoped thread creation and joining
- [Fork/Join](src/fork_join.rs) - non-scoped thread fork/joining
- [Scoped](src/scoped.rs) - scoped thread creation and MPSC example
- [Arc](src/arc.rs) - using `Arc` to share a value between non-scoped threads

## Links

- [Rust Book - Using Threads to Run Code Simultaneously](https://doc.rust-lang.org/book/ch16-01-threads.html)
- [Using Rust scoped threads to improve efficiency and safety](https://blog.logrocket.com/using-rust-scoped-threads-improve-efficiency-safety/)

## Questions

> Q. How does a join timeout work?

A. There is no join timeout in the Rust standard library.

For non-scoped threads, joining is optional, and if the main thread exits
without joining, all threads are automatically terminated. You can write
your own timeout if you want by using `JoinHandle::is_finished()`.

For scoped threads, the scope cannot be left until all the threads have
finished - if the scope closure "returns" early then it blocks until all
spawned threads have joined. If it calls `exit()` then the entire program
stops and all threads are killed immediately.

> Q. How does borrowing work across threads?

For non-scoped threads, if you want to share by borrowing a variable between
threads you need to use an `Arc<T>` and move the value to the heap. Otherwise
you have to move copies of the variable to each thread. Copies are required
because once you move a copy to a thread, it can't be moved to another thread.

This works out for `Arc` because you can clone `Arc` and it just increments
the ref counter, without duplicating the shared value on the heap.

Copies are required because the thread could last longer than the function that
starts it (and perhaps owns the variable). Therefore, moving is mandatory to
ensure the borrowed value outlives the thread.

For scoped threads, you have more flexibility, and can more easily share a
variable between threads without moving. I.e. the outer scope can continue
to own the variable, and the thread closures will just take shared borrows.
This also keeps the variable on the stack, without requiring a move to the heap.

This is allowed because the scope is guaranteed to last at least as long as
the threads (unless `exit()` is called), even if the main thread returns from
the scope (the join is automatic and can't be bypassed). If the main thread
exits then the entire program stops, which is still sound.

Note that for non-`Sync` types, you still need to move the variable into the
thread closure, because borrowing across threads is not allowed for non-Sync types.

> Q. If the main thread dies when the worker thread is borrowing, what happens?

A. For non-scoped threads, if the main thread dies (i.e. exits) then all
threads are immediately terminated. There is no opportunity for the worker
thread to continue. Remember that the worker thread must take ownership
of anything it borrows from (otherwise: "error[E0373]: closure may outlive
the current function, but it borrows x, which is owned by the current function").
So for non-scoped threads there isn't really borrowing happening across threads at all.

For scoped threads, a thread can borrow from the main thread (well, from the scope),
and this is fine because the scope is guaranteed to last at least as long as all the
spawned threads, as discussed above.
