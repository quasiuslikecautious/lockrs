use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/src/package.js")]
extern "C" {
    fn TailwindMerge(classlist1: &str, classlist2: &str) -> String;
}

#[wasm_bindgen]
pub fn cn(classlist1: &str, classlist2: &str) -> String {
    TailwindMerge(classlist1, classlist2)
}
