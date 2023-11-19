use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

#[derive(serde::Serialize)]
struct Person {
    pub name: String,
    pub surname: String,
    age: i32,
}

impl Person {
    fn new(name: String, surname: String, age: i32) -> Self {
        Person { name, surname, age }
    }
}

#[wasm_bindgen]
pub fn create_person(name: String, surname: String, age: i32) -> JsValue {
    let person = Person::new(name, surname, age);
    serde_wasm_bindgen::to_value(&person).unwrap()
}
