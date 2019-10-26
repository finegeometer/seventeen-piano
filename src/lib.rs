#![forbid(unsafe_code)]

use std::cell::RefCell;

use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen]
pub fn run() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    State::new();
}

#[derive(Clone)]
struct State(Rc<RefCell<Model>>);

struct Model {
    piano_keys: [Key; 35],
    document: web_sys::Document,
}

pub enum Msg {
    KeyDown(String),
    KeyUp(String),
}

impl State {
    fn new() -> Self {
        let out = Self(Rc::new(RefCell::new(Model::new())));

        {
            let model: &mut Model = &mut out.0.borrow_mut();
            out.event_listener(&model.document, "keydown", |evt| {
                let evt = evt.dyn_into::<web_sys::KeyboardEvent>().unwrap_throw();
                Msg::KeyDown(evt.key())
            });
            out.event_listener(&model.document, "keyup", |evt| {
                let evt = evt.dyn_into::<web_sys::KeyboardEvent>().unwrap_throw();
                Msg::KeyUp(evt.key())
            });
        }

        out
    }
    fn update(&self, msg: Msg) {
        let model: &mut Model = &mut self.0.borrow_mut();
        match msg {
            Msg::KeyDown(k) => {
                if let Some(n) = piano_key(&k) {
                    model.piano_keys[n].press();
                }
            }
            Msg::KeyUp(k) => {
                if let Some(n) = piano_key(&k) {
                    model.piano_keys[n].release()
                }
            }
        }
    }

    fn event_listener(
        &self,
        target: &web_sys::EventTarget,
        event: &str,
        msg: impl Fn(web_sys::Event) -> Msg + 'static,
    ) {
        let state = self.clone();
        let closure: Closure<dyn FnMut(web_sys::Event)> = Closure::wrap(Box::new(move |evt| {
            state.update(msg(evt));
        }));
        target
            .add_event_listener_with_callback(event, closure.as_ref().unchecked_ref())
            .unwrap_throw();
        closure.forget();
    }
}

impl Model {
    fn new() -> Self {
        let window = web_sys::window().unwrap_throw();
        let document = window.document().unwrap_throw();
        let body = document.body().unwrap_throw();

        let audio = web_sys::AudioContext::new().unwrap_throw();

        let grid = document
            .create_element("div")
            .unwrap_throw()
            .dyn_into::<web_sys::HtmlElement>()
            .unwrap_throw();
        grid.style().set_css_text(
            r"display: grid;
grid-gap: 10px;
background-color: #805020;
padding: 10px;
grid-template-areas:
    'xxx a02 a06 a09 a12 a16 a19 a23 a26 a29 a33'
    'xxx a02 a06 a09 a12 a16 a19 a23 a26 a29 a33'
    'xxx a02 a06 a09 a12 a16 a19 a23 a26 a29 a33'
    'xxx a03 a06 a09 a13 a16 a20 a23 a26 a30 a33'
    'a00 a03 a07 a10 a13 a17 a20 a24 a27 a30 a34'
    'a00 a03 a07 a10 a13 a17 a20 a24 a27 a30 a34'
    'a00 a04 a07 a10 a14 a17 a21 a24 a27 a31 a34'
    'a00 a04 a07 a10 a14 a17 a21 a24 a27 a31 a34'
    'a01 a04 a08 a11 a14 a18 a21 a25 a28 a31 yyy'
    'a01 a05 a08 a11 a15 a18 a22 a25 a28 a32 yyy'
    'a01 a05 a08 a11 a15 a18 a22 a25 a28 a32 yyy'
    'a01 a05 a08 a11 a15 a18 a22 a25 a28 a32 yyy';

position: fixed;
top: 0;
left: 0;
right: 0;
bottom: 0;
",
        );
        body.append_child(&grid).unwrap_throw();

        let mut i = 0;
        let mut key = || {
            let html = document
                .create_element("div")
                .unwrap_throw()
                .dyn_into::<web_sys::HtmlElement>()
                .unwrap_throw();
            html.style()
                .set_property("grid-area", &format!("a{:02}", i))
                .unwrap_throw();
            grid.append_child(&html).unwrap_throw();

            let out = Key::new(i, audio.clone(), html);
            i += 1;
            out
        };

        let piano_keys = [
            key(),
            key(),
            key(),
            key(),
            key(),
            key(),
            key(),
            key(),
            key(),
            key(),
            key(),
            key(),
            key(),
            key(),
            key(),
            key(),
            key(),
            key(),
            key(),
            key(),
            key(),
            key(),
            key(),
            key(),
            key(),
            key(),
            key(),
            key(),
            key(),
            key(),
            key(),
            key(),
            key(),
            key(),
            key(),
        ];

        Self {
            piano_keys,
            document,
        }
    }
}

fn piano_key(key: &str) -> Option<usize> {
    Some(match &key.to_lowercase() as &str {
        "q" => 0,
        "a" => 0,
        "z" => 1,

        "2" => 2,
        "w" => 3,
        "s" => 4,
        "x" => 5,

        "3" => 6,
        "e" => 7,
        "d" => 7,
        "c" => 8,

        "4" => 9,
        "r" => 10,
        "f" => 10,
        "v" => 11,

        "5" => 12,
        "t" => 13,
        "g" => 14,
        "b" => 15,

        "6" => 16,
        "y" => 17,
        "h" => 17,
        "n" => 18,

        "7" => 19,
        "u" => 20,
        "j" => 21,
        "m" => 22,

        "8" => 23,
        "i" => 24,
        "k" => 24,
        "," => 25,

        "9" => 26,
        "o" => 27,
        "l" => 27,
        "." => 28,

        "0" => 29,
        "p" => 30,
        ";" => 31,
        "/" => 32,

        "-" => 33,
        "[" => 34,
        "'" => 34,

        _ => None?,
    })
}

struct Key {
    keynum: usize,
    ctx: web_sys::AudioContext,
    osc: Option<web_sys::OscillatorNode>,
    html: web_sys::HtmlElement,
}

impl Key {
    fn new(keynum: usize, ctx: web_sys::AudioContext, html: web_sys::HtmlElement) -> Self {
        let mut out = Self {
            keynum,
            ctx,
            osc: None,
            html,
        };
        out.release();
        out
    }

    fn press(&mut self) {
        self.html
            .style()
            .set_property("background-color", "#FF0000D0")
            .unwrap_throw();

        if self.osc.is_none() {
            let osc = create_oscillator(&self.ctx);
            osc.frequency()
                .set_value(220. * (2f32).powf(self.keynum as f32 / 17.));
            self.osc = Some(osc);
        }
    }

    fn release(&mut self) {
        if (4..=13).contains(&((self.keynum * 12) % 17)) {
            self.html
                .style()
                .set_property("background-color", "#000000D0")
                .unwrap_throw();
        } else {
            self.html
                .style()
                .set_property("background-color", "#FFFFFFD0")
                .unwrap_throw();
        }

        if let Some(osc) = self.osc.take() {
            osc.stop().unwrap_throw();
        }
    }
}

fn create_oscillator(ctx: &web_sys::AudioContext) -> web_sys::OscillatorNode {
    const REAL: [f32; 16] = [
        0.0, 1.0, 0.6, 0.5, 0.3, 0.1, 0.2, 0.2, 0.15, 0.15, 0.02, 0.1, 0.07, 0.05, 0.07, 0.01,
    ];
    const IMAG: [f32; 16] = [
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
    ];

    let osc = ctx.create_oscillator().unwrap_throw();
    let gain = ctx.create_gain().unwrap_throw();
    osc.connect_with_audio_node(&gain).unwrap_throw();
    gain.connect_with_audio_node(&ctx.destination())
        .unwrap_throw();

    let wave = ctx
        .create_periodic_wave(&mut REAL, &mut IMAG)
        .unwrap_throw();
    osc.set_periodic_wave(&wave);

    gain.gain().set_value(0.1);

    osc.start().unwrap_throw();

    osc
}
