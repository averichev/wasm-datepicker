mod utils;
mod datepicker;

use wasm_bindgen::prelude::*;
use web_sys::Element;
use yew::prelude::*;
use yew::html::IntoHtmlResult;
use yew::Renderer;
use crate::datepicker::Datepicker;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn mount_datepicker(element: Element) {
    Renderer::<Datepicker>::with_root(element).render();
}
