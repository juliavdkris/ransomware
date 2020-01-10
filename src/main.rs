#![allow(dead_code)]
#![allow(unused_variables)]
use sodiumoxide::crypto::stream::chacha20;


fn chacha_import(key_json: &String) -> chacha20::Key {
	serde_json::from_str(&key_json).unwrap()
}

fn chacha_export(key: &chacha20::Key) -> String {
	serde_json::to_string(&key).unwrap()
}



fn main() {
	let key = chacha_import(&String::from("[213,216,159,212,251,40,38,202,228,225,175,168,131,10,84,14,87,199,173,30,210,26,22,1,28,153,192,72,163,38,159,222]"));
	// let key: chacha20::Key = chacha20::gen_key();
	let nonce = chacha20::Nonce::from_slice(b"verygood").unwrap();

	let plaintext = b"some data";
	let ciphertext = chacha20::stream_xor(plaintext, &nonce, &key);
	let their_plaintext = chacha20::stream_xor(&ciphertext, &nonce, &key);

	println!("{:?}", ciphertext);
}