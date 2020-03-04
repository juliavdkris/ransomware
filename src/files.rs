use std::io;
use std::io::prelude::*;
use std::fs::File;


/*
	List the files in a given directory
	Example
		let files = list_files("dirname").unwrap();
*/
pub fn list_files(dirname: &'static str) -> io::Result<Vec<std::path::PathBuf>> {
	let entries = std::fs::read_dir(&dirname)?
		.map(|res| res.map(|e| e.path()))
		.collect::<Result<Vec<_>, io::Error>>()?;
	Ok(entries)
}


/*
	Read a file to a bytevector
	Example
		let text = files::read_file("test.txt").unwrap();
*/
pub fn read_file(filename: &str) -> io::Result<Vec<u8>> {
	let mut f = File::open(filename)?;
	let mut buffer = Vec::new();

	f.read_to_end(&mut buffer)?;
	Ok(buffer)
}


/*
	Write a bytevector to a file
	Example
		files::write_file("test.txt", &bytevector).expect("Error writing file.");
*/
pub fn write_file(filename: &str, content: &Vec<u8>) -> io::Result<()> {
	let mut buffer = File::create(filename)?;

	buffer.write(&content)?;
	Ok(())
}