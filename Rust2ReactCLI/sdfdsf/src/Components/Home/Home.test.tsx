//Home.test.txt

use super::*;

#[wasm_bindgen_test]
fn test_home() {
    let home = Home::new();
    assert_eq!(home.render().to_string(), "Home component content");
}
