use std::error::Error;

#[macro_use]
extern crate lazy_static;

pub fn really_complicated_code(a: u8, b: u8) -> Result<u8, Box<dyn Error>> {
    Ok(a + b)
}

pub mod io;
