use std::convert::TryFrom;
use std::ops::{AddAssign, Sub};
use web_sys::{Document, Element, HtmlElement, Node};
use chrono::{Duration, NaiveDate};
use chrono::Utc;
use chrono::Datelike;
use chrono::TimeZone;
use chrono::Weekday;
use chronoutil::shift_months;
use gloo::console::log;
use web_sys::js_sys::Function;
use web_sys::wasm_bindgen::closure::Closure;
use web_sys::wasm_bindgen::JsCast;
use web_sys::MouseEvent;
use yew::{Component, Context, Html, html};


pub struct Datepicker {
    current_date: NaiveDate,
}

impl Component for Datepicker {
    type Message = ();
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let current_date = chrono::offset::Local::now().date_naive();
        Datepicker { current_date }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let all_days_of_week: Vec<Weekday> = (0..7)
            .map(|i| Weekday::try_from(i).unwrap())
            .collect();
        let columns = all_days_of_week.iter().map(|n|
            html! {
                            <th>{self.weekday_number_to_string(n)}</th>
                         }
        ).collect::<Html>();


        let calendarize = calendarize::calendarize_with_offset(self.current_date, 1);

        let rows = calendarize.iter().map(|n| {
            let cells = n.iter().map(|cl| {
                html!{
                    <td>{cl}</td>
                }
            }).collect::<Html>();
            html! {
                <tr>
                {cells}
                </tr>
            }
        }).collect::<Html>();

        html! {
            <table>
                <thead>
                    <tr>
                        {columns}
                    </tr>
                </thead>
                <tbody>
                    {rows}
                </tbody>
            </table>
        }
    }
}

impl Datepicker {
    fn weekday_number_to_string(&self, weekday: &Weekday) -> &'static str {
        match weekday {
            Weekday::Mon => "Пн",
            Weekday::Tue => "Вт",
            Weekday::Wed => "Ср",
            Weekday::Thu => "Чт",
            Weekday::Fri => "Пт",
            Weekday::Sat => "Сб",
            Weekday::Sun => "Вс",
        }
    }


    // pub fn render_in(&self, element: &Element) {
    //     let calendarize = calendarize::calendarize_with_offset(self.current_date, 1);
    //     let table = self.document.create_element("table").unwrap();
    //     let tr = self.document.create_element("tr").unwrap();
    //     let thead = self.document.create_element("thead").unwrap();
    //     let th = self.document.create_element("th").unwrap();
    //     let prev = self.prev_button();
    //     th.set_attribute("colspan", "7").unwrap();
    //     th.append_child(&prev).unwrap();
    //     let controls_tr = self.document.create_element("tr").unwrap();
    //     controls_tr.append_child(&th);
    //     thead.append_child(&controls_tr);
    //     thead.append_child(&tr);
    //
    //     let all_days_of_week: Vec<Weekday> = (0..7)
    //         .map(|i| Weekday::try_from(i).unwrap())
    //         .collect();
    //     for day in all_days_of_week {
    //         let td = self.document.create_element("th").unwrap();
    //         td.set_inner_html(self.weekday_number_to_string(day));
    //         tr.append_child(&td).unwrap();
    //     }
    //
    //     table.append_child(&thead);
    //
    //     let tbody = self.document.create_element("tbody").unwrap();
    //     for x in calendarize {
    //         let tr = self.document.create_element("tr").unwrap();
    //         for i in x {
    //             let td = self.document.create_element("td").unwrap();
    //             let mut date = String::new();
    //             if i > 0 {
    //                 date = i.to_string();
    //             }
    //             td.set_inner_html(&date);
    //             tr.append_child(&td);
    //         }
    //         tbody.append_child(&tr);
    //     }
    //     table.append_child(&tbody);
    //
    //
    //     element.set_inner_html("");
    //     element.append_child(&table);
    // }

    // fn prev_button(&self) -> Element {
    //     let prev = self.document.create_element("button").unwrap();
    //     let html = prev.clone().dyn_into::<HtmlElement>().unwrap();
    //     let clone = self.clone();
    //     let a = Closure::<dyn FnMut()>::new(move || {
    //         clone.handle_click();
    //     });
    //     html.set_onclick(Some(a.as_ref().unchecked_ref()));
    //     a.forget();
    //     //prev.add_event_listener_with_callback("click", );
    //     prev.set_inner_html("<");
    //
    //     prev
    // }

    // fn handle_click(&self) {
    //     log!("button clicked!")
    // }
}


