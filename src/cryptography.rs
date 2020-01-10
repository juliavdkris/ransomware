use sodiumoxide::crypto::stream::chacha20;
use base64;



pub fn chacha_import(key_base64: &'static str) -> chacha20::Key {
	let decoded = base64::decode(&key_base64).unwrap();
	chacha20::Key::from_slice(&decoded).unwrap()
}

pub fn chacha_export(key: &chacha20::Key) -> String {
	// serde_json::to_string(&key).unwrap()
	base64::encode(&key)
}