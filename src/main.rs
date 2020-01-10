#![allow(dead_code)]
#![allow(unused_variables)]
use sodiumoxide::crypto::stream::chacha20;

mod cryptography;
mod files;



fn main() {
	let key = cryptography::chacha_import("[213,216,159,212,251,40,38,202,228,225,175,168,131,10,84,14,87,199,173,30,210,26,22,1,28,153,192,72,163,38,159,222]");
	let nonce = chacha20::Nonce::from_slice(b"verygood").unwrap();

	let plaintext = files::read_file("victim_files/test.txt").unwrap();
	let ciphertext = chacha20::stream_xor(&plaintext, &nonce, &key);
	// let their_plaintext = chacha20::stream_xor(&ciphertext, &nonce, &key);

	println!("{:?}\n", plaintext);
	println!("{:?}\n", ciphertext);
	// println!("{:?}", their_plaintext);

	files::write_file("victim_files/test.txt.encrypted", ciphertext).expect("Error writing file.");
}