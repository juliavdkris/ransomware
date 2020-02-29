use std::collections::HashMap;


/*
	Sends a GET request, returns a result that can be unwrapped
	Example
		let body = networking::get("https://example.com").unwrap();
*/
pub fn get(url: &'static str) -> Result<String, Box<dyn std::error::Error>> {
	let body = reqwest::blocking::get(url)?
		.text()?;
	Ok(body)
}


/*
	Sends a POST request, doesn't return anything
	Example
		post("https://example.com", vec![("key", "value")])
*/
pub fn post(url: &'static str, map: &HashMap<&str, String>) {
	let client = reqwest::blocking::Client::new();
	client.post(url)
		.json(&map)
		.send()
		.expect("An error occurred while trying to send a POST request");
}