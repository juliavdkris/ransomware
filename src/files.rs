use std::io;
use std::io::prelude::*;
use std::fs::File;



pub fn read_file(filename: &'static str) -> io::Result<Vec<u8>> {
	let mut f = File::open(filename)?;
	let mut buffer = Vec::new();

	// read the whole file
	f.read_to_end(&mut buffer)?;
	Ok(buffer)
}

pub fn write_file(filename: &'static str, content: Vec<u8>) -> io::Result<()> {
	let mut buffer = File::create(filename)?;

	// Writes some prefix of the byte string, not necessarily all of it.
	buffer.write(&content)?;
	Ok(())
}