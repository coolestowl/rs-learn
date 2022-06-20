use std::{alloc::LayoutError};

use clap::{Args};

#[derive(Args)]
pub struct Sub {
    arg1: Option<String>
}

pub fn run(argv: &Sub) -> Result<(), LayoutError> {
    if let Some(arg) = &argv.arg1 {
        println!("in sub command, arg = {:?}", arg);
    }
    
    Ok(())
}
