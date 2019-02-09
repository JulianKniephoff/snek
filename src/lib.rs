#![feature(box_syntax)]
#![feature(try_from)]

extern crate wasm_bindgen;
extern crate web_sys;
extern crate console_error_panic_hook;

mod screen;

use std::{cell::RefCell, rc::Rc, collections::VecDeque};
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{
    HtmlCanvasElement,
    Window,
    KeyboardEvent,
};
use screen::{Screen};

struct State {
    board_size: (f64, f64),
    segments: VecDeque<Segment>,
    position: (f64, f64),
}

impl Orientation {
    fn to_direction<T: From<i8>>(&self) -> (T, T) {
        match self {
            Orientation::North => (T::from(0), T::from(-1)),
            Orientation::East => (T::from(1), T::from(0)),
            Orientation::South => (T::from(0), T::from(1)),
            Orientation::West => (T::from(-1), T::from(0)),
        }
    }
}

impl State {
    fn new(board_width: f64, board_height: f64) -> Self {
        assert!(board_width >= 0.0);
        assert!(board_height >= 0.0);
        const starting_length: usize = 5;
        assert!(board_width > starting_length as f64);
        let mut segments = VecDeque::new();
        segments.push_back(Segment::new(starting_length, Orientation::East));
        State {
            board_size: (board_width, board_height),
            position: ((starting_length - 1) as f64 + 10.0, 0.0 + 10.0),
            segments: segments,
        }
    }

    fn update(&mut self) {
        self.segments.front_mut().unwrap().length += 1;
        self.segments.back_mut().unwrap().length -= 1;
        if self.segments.back().unwrap().length == 0 {
            self.segments.pop_back();
        }
        let direction = self.segments
            .front().unwrap()
            .orientation
            .to_direction::<f64>();
        self.position.0 += direction.0;
        self.position.1 += direction.1;
        if self.position.0 < 0.0
            || self.position.0 >= self.board_size.0
            || self.position.1 < 0.0
            || self.position.1 >= self.board_size.1
        {
            panic!("Game Over!");
        }
    }
}

struct Segment {
    orientation: Orientation,
    length: usize,
}

impl Segment {
    fn new(length: usize, orientation: Orientation) -> Self {
        Segment { length, orientation }
    }
}

enum Orientation {
    North,
    East,
    South,
    West,
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
            state.segments.push_back(Segment::new(5, Orientation::North));
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
    let context = screen.context();
    context.save();
    context.translate(0.5, 0.5);
    context.set_line_cap("square");
    let mut position = state.position;
    for (i, segment) in state.segments.iter().enumerate() {
        context.begin_path();
        context.save();
        context.set_stroke_style(&["green", "red"][i % 2].into());
        context.move_to(position.0, position.1);
        let direction = segment.orientation.to_direction::<f64>();
        position.0 -= direction.0 * (segment.length - 1) as f64;
        position.1 -= direction.1 * (segment.length - 1) as f64;
        context.line_to(position.0, position.1);
        position.0 -= direction.0;
        position.1 -= direction.1;
        context.stroke();
        context.restore();
    }
    context.restore();

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
