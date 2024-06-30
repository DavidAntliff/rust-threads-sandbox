#![allow(dead_code)]

mod arc;
mod basic;
mod fork_join;
mod scoped;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Non-scoped:");
    basic::main()?;

    println!("Scoped:");
    scoped::main()?;

    println!("Scoped MPSC:");
    scoped::mpsc()?;

    println!("Non-scoped Arc:");
    arc::main()?;

    println!("Fork/Join:");
    fork_join::main()?;

    Ok(())
}
