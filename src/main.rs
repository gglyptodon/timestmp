use chrono::offset::Local;
use std::error::Error;
use std::{io, io::prelude::*};
pub type DynResult<T> = Result<T, Box<dyn Error>>;

pub fn main() -> DynResult<()> {
    for line in io::stdin().lock().lines() {
        let line = line?;
        println!("# length = {} @ {} #\n{}", line.len(), Local::now(), line);
    }
    Ok(())
}
