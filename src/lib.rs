use wasm_bindgen::prelude::*;


#[wasm_bindgen]
pub fn greet(name: &str) -> JsValue {
    let str = &format!("Hello, {}!", name);
    JsValue::from_str(str)
}

// cargo install wasm-pack
// wasm-pack build --target nodejs
// https://rustwasm.github.io/docs/wasm-bindgen/examples/fetch.html