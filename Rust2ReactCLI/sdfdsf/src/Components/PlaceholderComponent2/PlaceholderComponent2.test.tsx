//PlaceholderComponent2.test.txt

use super::*;

#[wasm_bindgen_test]
fn test_placeholder_component_2() {
    let placeholder_component_2 = PlaceholderComponent2::new();
    assert_eq!(
        placeholder_component_2.render().to_string(),
        "PlaceholderComponent2 content"
    );
}
