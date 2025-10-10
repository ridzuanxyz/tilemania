use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn test_wolges_basic() -> bool {
    // Basic test: wolges compiles to WASM
    true
}

#[wasm_bindgen]
pub fn validate_word(word: String) -> bool {
    // Will implement with KWG in afternoon
    word.len() > 0
}

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn test_basic() {
        assert!(super::test_wolges_basic());
    }
}
