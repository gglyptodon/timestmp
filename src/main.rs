use std::{io, io::prelude::*};
use std::error::Error;
use chrono::offset::Local;
pub type DynResult<T> = Result<T, Box<dyn Error>>;

pub fn main() -> DynResult<()> {
    for line in io::stdin().lock().lines() {
        let line = line?;
        println!("# length = {} @ {} #\n{}", line.len(), Local::now(), line);
    }
    Ok(())
}

//pub fn get_current_timestamp() -> String {
//    format!("{}", chrono::offset::Local::now())
//}
