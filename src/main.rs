extern crate byteorder;
#[macro_use]
extern crate clap;

use clap::{App, Arg};
 
fn main() { 
    let matches = App::new("chipmemify")
        .version("0.1")
        .about("Moves an amiga data block to chipmem")
        .author("mop")
        .arg(Arg::with_name("file")
            .required(true)
            .value_name("FILE")
            .help("amiga executable")
            .takes_value(true))
        .arg(Arg::with_name("datalen")
            .required(true)
            .value_name("DATALEN")
            .help("length of data section to chipmemify")
            .takes_value(true))
        .get_matches();

    let len = value_t!(matches, "datalen", u32).unwrap_or_else(|e| println!("{}", e));
    
}