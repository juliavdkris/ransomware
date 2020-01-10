#![allow(dead_code)]
#![allow(unused_variables)]
use sodiumoxide::crypto::stream::chacha20;

mod cryptography;
mod files;



fn main() {
	let key = cryptography::chacha_import("1dif1PsoJsrk4a+ogwpUDlfHrR7SGhYBHJnASKMmn94=");
	let nonce = chacha20::Nonce::from_slice(b"verygood").unwrap();

	let plaintext = files::read_file("victim_files/test.txt").unwrap();
	let ciphertext = chacha20::stream_xor(&plaintext, &nonce, &key);
	// let their_plaintext = chacha20::stream_xor(&ciphertext, &nonce, &key);

	println!("{:?}\n", plaintext);
	println!("{:?}\n", ciphertext);
	// println!("{:?}", their_plaintext);

	files::write_file("victim_files/test.txt.encrypted", ciphertext).expect("Error writing file.");
}