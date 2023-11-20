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
#[derive(Clone)]
pub struct Datepicker {
    element: Element,
    on_select: Option<Function>
}

#[wasm_bindgen]
impl Datepicker {
    pub fn new(element: Element) -> Datepicker {
        Datepicker { element, on_select: None }
    }
    pub fn with_callback(&mut self, on_select: Function) -> Datepicker{
        self.on_select = Some(on_select);
        (*self).clone()
    }
    pub fn render(&self) {
        Renderer::<WrapperComponent>::with_root_and_props(
            self.element.clone(),
            WrapperComponentProperties { on_select: self.on_select.clone() },
        )
        .render();
    }
}

pub struct WrapperComponent;

#[derive(Default, Properties, PartialEq)]
pub struct WrapperComponentProperties {
    on_select: Option<Function>,
}

impl Component for WrapperComponent {
    type Message = ();
    type Properties = WrapperComponentProperties;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let callback = ctx.props().on_select.clone();
        let on_select = move |date: NaiveDate| {
            let date_js = Date::new_with_year_month_day(
                date.year() as u32,
                date.month0().try_into().unwrap(),
                date.day() as i32,
            );
            match callback.clone() {
                None => {}
                Some(fnc) => {
                    fnc.call1(&JsValue::NULL, &date_js).unwrap();
                }
            }
        };

        html! {
            <DatepickerComponent {on_select} />
        }
    }
}
