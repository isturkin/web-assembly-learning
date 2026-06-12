use wasm_bindgen::prelude::*;

#[wasm_bindgen] // вызов этой функции приведет к вызову JS-реализации функции
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen] // для данной функции сгенерируется bridge-функция для JavaScript
pub fn say_hello(name: &str, whom: &str) {
    alert(&format!("Hello, {} from {}!", name, whom))
}

// #[unsafe(no_mangle)] // WebAssembly test
pub extern "C" fn add(x: i32, y: i32) -> i32 {
    x + y
}
