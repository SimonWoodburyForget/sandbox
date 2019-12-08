

#![allow(clippy::non_ascii_literal)]

#[macro_use]
extern crate seed;
use seed::prelude::*;

struct Model {
    count: i32,
    words: String
}

impl Default for Model {
    fn default() -> Self {
        Self {
            count: 0,
            words: "string".into()
        }
    }
}

#[derive(Debug, Clone)]
enum Msg {
    Increment,
    Decrement,
    Filter(String)
}

fn success_level(clicks: i32) -> Node<Msg> {
    let descrip = match clicks {
        0 ..= 5 => "Not very many 🙁",
        6 ..= 9 => "I got my first real six-string 😐",
        10 ..= 11 => "Spinal Tap 🙂",
        _ => "Double pendulum 🙃"
    };
    p![ descrip ]
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::Increment => model.count += 1,
        Msg::Decrement => model.count -= 1,
        Msg::Filter(words) => model.words = words,
    }
}

fn view(model: &Model) -> impl View<Msg> {
    div![
        button![simple_ev(Ev::Click, Msg::Increment), "+"],
        div![format!("{}", model.count)],
        button![simple_ev(Ev::Click, Msg::Decrement), "-"],
        success_level(model.count)
    ]
}

#[wasm_bindgen(start)]
pub fn render() {
    seed::App::builder(update, view).build_and_start();
}
