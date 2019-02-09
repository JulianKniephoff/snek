#![feature(box_syntax)]
#![feature(try_from)]

extern crate wasm_bindgen;
extern crate web_sys;
extern crate console_error_panic_hook;

mod screen;

use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{
    HtmlCanvasElement,
    Window,
    KeyboardEvent,
};
use screen::{Screen};

struct State {
    position: (f64, f64),
    direction: (f64, f64),
    board_size: (f64, f64),
}

impl State {
    fn new(board_width: f64, board_height: f64) -> Self {
        assert!(board_width >= 0.0);
        assert!(board_height >= 0.0);
        State {
            board_size: (board_width, board_height),
            position: (0.0, 0.0),
            direction: (1.0, 0.0),
        }
    }

    fn update(&mut self) {
        self.position.0 += self.direction.0;
        self.position.1 += self.direction.1;
        if self.position.0 < 0.0
            || self.position.0 >= self.board_size.0
            || self.position.1 < 0.0
            || self.position.1 >= self.board_size.1
        {
            self.position = (0.0, 0.0);
            self.direction = (1.0, 0.0);
        }
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    snek();
}

fn snek() {
    console_error_panic_hook::set_once();

    const BOARD_WIDTH: f64 = 100.0;
    const BOARD_HEIGHT: f64 = 100.0;

    let state = Rc::new(RefCell::new(State::new(BOARD_WIDTH, BOARD_HEIGHT)));
    let input_state = state.clone();

    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();

    let style = body.style();
    style.set_property("margin", "0").unwrap();
    style.set_property("overflow", "hidden").unwrap();

    let canvas: HtmlCanvasElement = document.create_element("canvas")
        .unwrap().unchecked_into();
    body.append_child(&canvas).unwrap();

    const FRAME_WIDTH: usize = BOARD_WIDTH as usize;
    const FRAME_HEIGHT: usize = BOARD_HEIGHT as usize;
    fit_canvas(&window, &canvas, (FRAME_WIDTH, FRAME_HEIGHT));

    let screen = Rc::new(Screen::new(canvas, FRAME_WIDTH, FRAME_HEIGHT));
    let resize_screen = screen.clone();

    let mut previous_time = window.performance().unwrap().now();
    let mut lag = 0.0;

    let main_loop = Rc::new(RefCell::new(None));
    let main_loop_cont = main_loop.clone();
    *main_loop.borrow_mut() = Some(Closure::wrap((box move |time: f64| {

        const TIME_STEP: f64 = 1000.0;

        lag += time - previous_time;
        previous_time = time;

        if lag >= TIME_STEP {
            state.borrow_mut().update();
            lag = 0.0;

            render(&state.borrow(), &screen);
        }

        web_sys::window().unwrap().request_animation_frame(
            (
                main_loop_cont
                    .borrow()
                    .as_ref()
                    .unwrap() as &Closure<_>
            )
                .as_ref()
                .unchecked_ref()
        ).unwrap();
    }) as Box<dyn FnMut(_)>));
    window.request_animation_frame(
        main_loop.borrow().as_ref().unwrap().as_ref().unchecked_ref()
    ).unwrap();

    let handler = Closure::wrap(
        (box move |event: KeyboardEvent| {
            let mut state = input_state.borrow_mut();
            match event.key().as_ref() {
                "ArrowLeft" => if state.direction.1 != 0.0 {
                    state.direction = (-1.0, 0.0);
                },
                "ArrowRight" => if state.direction.1 != 0.0 {
                    state.direction = (1.0, 0.0);
                },
                "ArrowUp" => if state.direction.0 != 0.0 {
                    state.direction = (0.0, -1.0);
                }
                "ArrowDown" => if state.direction.0 != 0.0 {
                    state.direction = (0.0, 1.0);
                }
                _ => (),
            }
        }) as Box<dyn FnMut(_)>,
    );
    window.add_event_listener_with_callback(
        "keyup",
        handler.as_ref().unchecked_ref(),
    ).unwrap();
    handler.forget();

    let handler = Closure::wrap(
        (box move || {
            let window = web_sys::window().unwrap();
            fit_canvas(
                &window,
                window.document().unwrap()
                    .get_elements_by_tag_name("canvas")
                    .item(0).unwrap()
                    .unchecked_ref(),
                (FRAME_WIDTH, FRAME_HEIGHT),
            );
            resize_screen.resize();
        }) as Box<dyn FnMut()>,
    );
    window.add_event_listener_with_callback(
        "resize",
        handler.as_ref().unchecked_ref(),
    ).unwrap();
    handler.forget();
}

fn render(state: &State, screen: &Screen) {
    screen.context().fill_rect(
        state.position.0,
        state.position.1,
        1.0,
        1.0,
    );
    screen.flip();
}

fn fit_canvas(
    window: &Window,
    canvas: &HtmlCanvasElement,
    virtual_size: (usize, usize),
) {
    let canvas_width = window.inner_width()
        .unwrap().as_f64().unwrap() as i32;
    let canvas_height = window.inner_height()
        .unwrap().as_f64().unwrap() as i32;
    assert!(canvas_width > 0);
    assert!(canvas_height > 0);
    let canvas_width = canvas_width as usize;
    let canvas_height = canvas_height as usize;
    canvas.set_width(canvas_width as u32);
    canvas.set_height(canvas_height as u32);
}
