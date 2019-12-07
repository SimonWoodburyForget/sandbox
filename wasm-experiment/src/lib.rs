use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> { Ok(()) }

#[wasm_bindgen]
pub fn add(a: u32, b: u32) -> u32 { a + b }

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
