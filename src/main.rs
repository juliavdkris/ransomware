use sodiumoxide::crypto::stream::xchacha20;
use base64;


fn main() {
	let key = xchacha20::gen_key();
	let nonce = xchacha20::gen_nonce();
	let plaintext = b"some data";
	let ciphertext = xchacha20::stream_xor(plaintext, &nonce, &key);
	let _their_plaintext = xchacha20::stream_xor(&ciphertext, &nonce, &key);

	println!("{}", base64::encode(&ciphertext));
}