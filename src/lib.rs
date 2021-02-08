#![feature(box_syntax)]
#![allow(clippy::wildcard_imports)]

use seed::{prelude::*, *};
use seed::prelude::js_sys::parse_float;

/// Initialize model with test values
fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model {
        l1: 1.,
        l2: 1.,
        l3: 1.,
        a1: 25.,
        a2: 310.,
        a3: 60.,
        o1: 4.,
        o2: -2.,
        o3: 6.,
    }
}

/// Variables
struct Model {
    // L in m
    l1: f64,
    l2: f64,
    l3: f64,
    // θ in deg
    a1: f64,
    a2: f64,
    a3: f64,
    // ω in rad/s
    o1: f64,
    o2: f64,
    o3: f64,
}

#[derive(Copy, Clone)]
enum Var {
    L1,
    L2,
    L3,
    A1,
    A2,
    A3,
    O1,
    O2,
    O3,
}


#[derive(Copy, Clone)]
enum Msg {
    UpdateVar(Var, f64),
}

// `update` describes how to handle each `Msg`.
fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::UpdateVar(var, val) => {
            let val = if val.is_nan() { 0. } else { val };
            match var {
                Var::L1 => model.l1 = val,
                Var::L2 => model.l2 = val,
                Var::L3 => model.l3 = val,
                Var::A1 => model.a1 = val,
                Var::A2 => model.a2 = val,
                Var::A3 => model.a3 = val,
                Var::O1 => model.o1 = val,
                Var::O2 => model.o2 = val,
                Var::O3 => model.o3 = val,
            }
        }
    }
}

#[allow(clippy::trivially_copy_pass_by_ref)]
/// Display to the page
fn view(model: &Model) -> Node<Msg> {
    // Make shortcut variables for legibility
    let Model {l1, l2, l3, a1, a2, a3, o1, o2, o3} = model;
    let da1 = a1;
    let da2 = a2;
    let da3 = a3;
    let a1 = a1.to_radians();
    let a2 = a2.to_radians();
    let a3 = a3.to_radians();
    let a12 = a1 + a2;
    let a123 = a12 + a3;
    // Positions
    let x = l1 * a1.cos() + l2 * a12.cos() + l3 * a123.cos();
    let y = l1 * a1.sin() + l2 * a12.sin() + l3 * a123.sin();
    let p = a123.to_degrees() - (a123.to_degrees() / 360.).round() * 360.;
    // Dot variables
    let ad1 = o1;
    let ad2 = o2;
    let ad3 = o3;
    let ad12 = ad1 + ad2;
    let ad123 = ad12 + ad3;
    // Velocities
    let xd = -l1 * ad1 * a1.sin() - l2 * ad12 * a12.sin() - l3 * ad123 * a123.sin();
    let yd = l1 * ad1 * a1.cos() + l2 * ad12 * a12.cos() + l3 * ad123 * a123.cos();
    div![
        C!["p-3"],
        h1![C!["text-4xl", "font-bold", "mb-3"], "3r: forward kinematics"],
        div![
            div![
                C!["flex flex-row flex-wrap space-x-4"],
              div![
                p![C!["font-bold", "mt-3"], "Lengths"],
                char_sub_val_un(Var::L1, l1),
                char_sub_val_un(Var::L2, l2),
                char_sub_val_un(Var::L3, l3),
              ],
              div![
                  p![C!["font-bold", "mt-3"], "Angles"],
                  char_sub_val_un(Var::A1, da1),
                  char_sub_val_un(Var::A2, da2),
                  char_sub_val_un(Var::A3, da3),
              ],
              div![
                  p![C!["font-bold", "mt-3"], "Velocities"],
                  char_sub_val_un(Var::O1, o1),
                  char_sub_val_un(Var::O2, o2),
                  char_sub_val_un(Var::O3, o3),
              ],
            ],
            p![C!["font-bold", "mt-3"], "Results"],
            p!["(x, y, ɸ): ", format!("({:.3} m, {:.3} m, {}º)", x, y, p)],
            p!["(v", sub!["x"], ", v",sub!["y"], ", ɸ̇) : ", format!("({:.3} m/s, {:.3} m/s, {} rad/s)", xd, yd, ad123)],
        ],
        footer_view(),
    ]
}

/// The footer after main content
fn footer_view() -> Node<Msg> {
    footer![
        p![
            C!["opacity-80", "mt-10"],
            "made by ",
            a![
                C!["text-indigo-500", "font-medium"],
                attrs!{
                    At::Href => "https://birla.io"
                },
                "louis birla",
              ],
            " to explore rust on the frontend"
          ]
    ]
}

fn char_sub_val_un(var: Var, val: &f64) -> Node<Msg> {
    let char = match var {
        Var::L1 | Var::L2 | Var:: L3 => "L",
        Var::A1 | Var::A2 | Var:: A3 => "θ",
        Var::O1 | Var::O2 | Var:: O3 => "ω",
    };
    let unit = match var {
        Var::L1 | Var::L2 | Var:: L3 => " m",
        Var::A1 | Var::A2 | Var:: A3 => "º",
        Var::O1 | Var::O2 | Var:: O3 => " rad/s",
    };
    let sub = match var {
        Var::L1 | Var::A1 | Var:: O1 => "1",
        Var::L2 | Var::A2 | Var:: O2 => "2",
        Var::L3 | Var::A3 | Var:: O3 => "3",
    };
    p![
        char,
        sub![sub],
        " = ",
        input![
            C!["w-10", "border", "my-1", "rounded-lg", "pl-2", "border-indigo-400", "ring-indigo-500"],
            attrs! {
              At::Value => val,
            },
            input_ev(Ev::Input, move |val| Msg::UpdateVar(var, parse_float(&val))),
        ],
        unit,
    ]
}

#[wasm_bindgen(start)]
pub fn start() {
    // Mount the `app` to the element with the `id` "app".
    App::start("app", init, update, view);
}
