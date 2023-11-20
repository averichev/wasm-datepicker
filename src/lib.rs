use chrono::{Datelike, NaiveDate};
use std::convert::TryInto;
use wasm_bindgen::prelude::*;
use web_sys::js_sys::{Date, Function};
use web_sys::Element;
use yew::prelude::*;
use yew::Renderer;
use yew_datepicker::Datepicker as DatepickerComponent;

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
            WrapperComponentProperties { callback },
        )
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
