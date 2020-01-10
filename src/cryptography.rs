use sodiumoxide::crypto::stream::chacha20;


pub fn chacha_import(key_json: &'static str) -> chacha20::Key {
	serde_json::from_str(&key_json).unwrap()
}

pub fn chacha_export(key: &chacha20::Key) -> String {
	serde_json::to_string(&key).unwrap()
}