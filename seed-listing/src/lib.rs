#![allow(clippy::non_ascii_literal)]

#[macro_use]
extern crate seed;

use futures::Future;
use seed::prelude::*;
use seed::{fetch, Method, Request};
use serde::{Deserialize, Serialize};

struct Model {
    count: i32,
    words: String,
    data: Vec<f64>,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            count: 0,
            words: "string".into(),
            data: vec![],
        }
    }
}

#[derive(Debug, Clone)]
enum Msg {
    Increment,
    Decrement,
    FetchData(fetch::ResponseDataResult<Vec<f64>>),
}

fn fetch_data() -> impl Future<Item = Msg, Error = Msg> {
    Request::new("data/test.json")
        .method(Method::Get)
        .fetch_json_data(Msg::FetchData)
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Increment => {
            model.count += 2;
            orders.skip().perform_cmd(fetch_data());
        }

        Msg::Decrement => model.count -= 1,

        Msg::FetchData(data) => {
            model.data = data.unwrap();
        }
    }
}

fn view(model: &Model) -> impl View<Msg> {
    div![
        button![simple_ev(Ev::Click, Msg::Increment), "+"],
        div![format!("{}", model.count)],
        button![simple_ev(Ev::Click, Msg::Decrement), "-"],
        p![format!("{:?}", model.data)],
    ]
}

#[wasm_bindgen(start)]
pub fn render() {
    seed::App::builder(update, view).build_and_start();
}
