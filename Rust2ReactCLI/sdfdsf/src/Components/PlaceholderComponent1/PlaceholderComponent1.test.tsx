//PlaceholderComponent1.test.txt

use super::*;

#[wasm_bindgen_test]
fn test_placeholder_component_1() {
    let placeholder_component_1 = PlaceholderComponent1::new();
    assert_eq!(
        placeholder_component_1.render().to_string(),
        "PlaceholderComponent1 content"
    );
}
