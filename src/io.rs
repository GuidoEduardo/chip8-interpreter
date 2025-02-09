use wasm_bindgen::prelude::*;
use wasm_bindgen::closure::Closure;
use web_sys::{ KeyboardEvent };

#[wasm_bindgen(start)]
pub fn run() {
    let window = web_sys::window().expect("no global `window` exists");
    let document = web_sys::window().unwrap().document().expect("should have a document on window");

    let keydown_closure = Closure::wrap(Box::new(move |event: KeyboardEvent| {
        handle_keydown(event);
    })) as Box<dyn FnMut(KeyboardEvent)>;

    document.add_event_listener_with_callback(
        "keydown",
        keydown_closure.as_ref().unchecked_ref()
    ).expect("could not add keydown listener to document");

    keydown_closure.forget();
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

pub fn handle_keydown(event: KeyboardEvent) {
    log(&format!("Tecla pressionada: {}", event.key()).into());
}
