use wasm_bindgen::prelude::*;

#[cfg(test)]
mod tests;

pub mod token;
pub mod utils;

#[wasm_bindgen]
extern "C" {
	pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
	alert(&format!("Hello, {}!", name));
}
