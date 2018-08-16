extern crate byteorder;
#[macro_use]
extern crate clap;

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use clap::{App, Arg};
use std::fs::{OpenOptions};
use std::io::{Error, ErrorKind, Seek, SeekFrom};
use std::path::Path;

// large parts stolen from here:
// https://github.com/emoon/amiga_hunk_parser/blob/master/src/lib.rs

const HUNK_HEADER: u32 = 1011;

const HUNKF_CHIP: u32 = 1 << 30;
const HUNKF_FAST: u32 = 1 << 31;

#[derive(Clone, Copy, Debug)]
pub enum MemoryType {
    Any,
    Chip,
    Fast,
}

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

    let datalen = value_t_or_exit!(matches.value_of("datalen"), usize);
    let file = matches.value_of("file").unwrap();

    match chipmemify(file, datalen) {
        Ok(()) => println!("Great success"),
        Err(e) => eprintln!("Fail: {}", e),
    };
}

fn get_size_type(t: u32) -> (usize, MemoryType) {
        let size = (t & 0x0fffffff) * 4;
        let mem_t = t & 0xf0000000;
        let mem_type = match mem_t {
            HUNKF_CHIP => MemoryType::Chip,
            HUNKF_FAST => MemoryType::Fast,
            _ => MemoryType::Any,
        };

        (size as usize, mem_type)
}

fn chipmemify(file: &str, datalen: usize) -> Result<(), Error> {
    let path = Path::new(file);
    let mut file = OpenOptions::new().read(true).write(true).open(path)?;

    let hunk_header = file.read_u32::<BigEndian>()?;
    if hunk_header != HUNK_HEADER  {
        return Err(Error::new(ErrorKind::Other, "Unable to find correct HUNK_HEADER"));
    };

    // Skip header/string section
    file.read_u32::<BigEndian>()?;

    let table_size = file.read_u32::<BigEndian>()? as i32;
    let first_hunk = file.read_u32::<BigEndian>()? as i32;
    let last_hunk = file.read_u32::<BigEndian>()? as i32;

    if table_size < 0 || first_hunk < 0 || last_hunk < 0 {
        return Err(Error::new(ErrorKind::Other, "Invalid sizes for hunks"));
    }

    let hunk_count = (last_hunk - first_hunk + 1) as usize;

    for i in 0..hunk_count {
        let (size, _) = get_size_type(file.read_u32::<BigEndian>()?);
        if size == datalen {
            let offset = (5+i) * 4;
            file.seek(SeekFrom::Start(offset as u64))?;
            file.write_u32::<BigEndian>(((size >> 2) as u32 | 0x40000000) & 0x4fffffff)?;
            println!("Hurra");
        }
    }
    Ok(())
}