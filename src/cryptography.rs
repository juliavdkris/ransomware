use sodiumoxide::crypto::stream::chacha20;

/*
	Import a ChaCha20 key from a base64 string
	Example
		let key = cryptography::chacha_import("1dif1PsoJsrk4a+ogwpUDlfHrR7SGhYBHJnASKMmn94=");
*/
pub fn chacha_import(key_base64: &str) -> chacha20::Key {
	let decoded = base64::decode(&key_base64).unwrap();
	chacha20::Key::from_slice(&decoded).unwrap()
}

/*
	Export a ChaCha20 key to a base64 string
	Example
		let key_base64 = cryptography::chacha_export(&key);
*/
pub fn chacha_export(key: &chacha20::Key) -> String {
	base64::encode(&key)
}
