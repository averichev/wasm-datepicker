mod utils;
pub mod datepicker;

use std::convert::TryInto;
use chrono::{Datelike, NaiveDate};
use gloo::console::log;
use wasm_bindgen::__rt::IntoJsResult;
use wasm_bindgen::prelude::*;
use web_sys::{Element};
use web_sys::js_sys::{Date, Function};
use yew::prelude::*;
use yew::html::IntoHtmlResult;
use yew::Renderer;
use crate::datepicker::Datepicker as DatepickerComponent;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
#[derive(PartialEq)]
pub struct DatepickerOptions {}

#[wasm_bindgen]
pub struct Datepicker {
    element: Element,
}

#[wasm_bindgen]
impl Datepicker {
    pub fn new(element: Element) -> Datepicker {
        Datepicker { element }
    }
    pub fn render(&self, callback: Function) {
        Renderer::<WrapperComponent>::with_root_and_props(
            self.element.clone(),
            WrapperComponentProperties { callback })
            .render();
    }
}

pub struct WrapperComponent;

#[derive(Properties, PartialEq)]
pub struct WrapperComponentProperties {
    callback: Function,
}

impl Component for WrapperComponent {
    type Message = ();
    type Properties = WrapperComponentProperties;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let callback = ctx.props().callback.clone();
        let on_select = move |date: NaiveDate| {
            let date_js = Date::new_with_year_month_day(
                date.year() as u32,
                date.month0().try_into().unwrap(),
                date.day() as i32,
            );
            callback.call1(&JsValue::NULL, &date_js).unwrap();
        };

        html! {
            <DatepickerComponent {on_select} />
        }
    }
}

impl WrapperComponent {
    fn trigger_on_select(&self, date: &NaiveDate) {}
}