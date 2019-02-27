#![feature(box_syntax)]
#![feature(try_from)]

extern crate rand;
extern crate wasm_bindgen;
extern crate web_sys;
extern crate console_error_panic_hook;

mod screen;

use std::{cell::RefCell, rc::Rc, collections::VecDeque};
use rand::{thread_rng, seq::SliceRandom};
use wasm_bindgen::{prelude::*, JsCast, convert::FromWasmAbi};
use web_sys::{
    HtmlCanvasElement,
    Window,
    KeyboardEvent,
    FocusEvent,
};
use screen::{Screen};

struct State {
    board_size: (f64, f64),
    segments: VecDeque<Segment>,
    food: (f64, f64),
    occupied: Vec<bool>,
    free_cells: Vec<(f64, f64)>,
    had_food: bool,
}

impl State {
    fn new(board_width: f64, board_height: f64) -> Self {
        assert!(board_width >= 0.0);
        assert!(board_height >= 0.0);
        const starting_length: usize = 5;
        assert!(board_width > starting_length as f64);
        let mut segments = VecDeque::new();
        let starting_position = ((starting_length - 1) as f64, 0.0);
        segments.push_back(Segment::new(
            starting_position,
            starting_length,
            Orientation::East
        ));
        let cell_count = board_width as usize * board_height as usize;
        let mut occupied = vec![false; cell_count];
        for x in 0..starting_length {
            occupied[x] = true;
        }
        let mut free_cells = Vec::with_capacity(cell_count);
        State {
            board_size: (board_width, board_height),
            food: State::spawn_food(
                (board_width as usize, board_height as usize),
                &occupied,
                &mut free_cells,
            ),
            segments,
            occupied,
            free_cells,
            had_food: false,
        }
    }

    fn update(&mut self) {
        let head_start = {
            let head = self.segments.front_mut().unwrap();
            let direction = head.orientation.to_direction::<f64>();
            head.start.0 += direction.0;
            head.start.1 += direction.1;
            head.start
        };

        if head_start.0 < 0.0
            || head_start.0 >= self.board_size.0
            || head_start.1 < 0.0
            || head_start.1 >= self.board_size.1
        {
            panic!("Game Over!");
        }

        if !self.had_food {
            let tail_end = {
                let tail = self.segments.back_mut().unwrap();
                let tail_end = tail.end;
                let direction = tail.orientation.to_direction::<f64>();
                tail.end.0 += direction.0;
                tail.end.1 += direction.1;
                if tail.start == tail.end {
                    self.segments.pop_back();
                }
                tail_end
            };
            self.occupy(tail_end, false);
        }

        if self.occupied[self.to_index(head_start)] {
            panic!("Game Over!");
        }

        self.occupy(head_start, true);

        self.had_food = false;

        if head_start == self.food {
            self.food = State::spawn_food(
                (self.board_size.0 as usize, self.board_size.1 as usize),
                &self.occupied,
                &mut self.free_cells,
            );
            self.had_food = true;
        }
    }

    fn spawn_food<'a>(
        board_size: (usize, usize),
        occupied: impl IntoIterator<Item = &'a bool>,
        free_cells: &mut Vec<(f64, f64)>,
    ) -> (f64, f64) {
        free_cells.clear();
        free_cells.extend(
            occupied.into_iter()
                .enumerate()
                .filter(|(_, &occupied)| !occupied)
                .map(|(index, _)| State::to_position(index, (
                    board_size.0 as f64,
                    board_size.1 as f64,
                )))
        );
        *free_cells.choose(&mut thread_rng()).unwrap()
    }

    fn to_position(index: usize, size: (f64, f64)) -> (f64, f64) {
        ((index % size.0 as usize) as f64, (index / size.0 as usize) as f64)
    }

    fn to_index(&self, position: (f64, f64)) -> usize {
        (position.1 * self.board_size.0 + position.0) as usize
    }

    fn occupy(&mut self, position: (f64, f64), occupied: bool) {
        let index = self.to_index(position);
        self.occupied[index] = occupied;
    }
}

struct Segment {
    orientation: Orientation,
    start: (f64, f64),
    end: (f64, f64),
}

impl Segment {
    fn new(start: (f64, f64), length: usize, orientation: Orientation) -> Self {
        let direction = orientation.to_direction::<f64>();
        Segment {
            orientation,
            start,
            end: (
                start.0 - direction.0 * length as f64,
                start.1 - direction.1 * length as f64,
            ),
        }
    }
}

enum Orientation {
    North,
    East,
    South,
    West,
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

#[wasm_bindgen(start)]
pub fn main() {
    snek();
}

fn snek() {
    console_error_panic_hook::set_once();

    const BOARD_WIDTH: f64 = 20.0;
    const BOARD_HEIGHT: f64 = 15.0;

    let state = Rc::new(RefCell::new(State::new(BOARD_WIDTH, BOARD_HEIGHT)));

    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();

    let style = body.style();
    style.set_property("margin", "0").unwrap();
    style.set_property("overflow", "hidden").unwrap();

    let canvas: HtmlCanvasElement = document.create_element("canvas")
        .unwrap().unchecked_into();
    body.append_child(&canvas).unwrap();
    fit_canvas(&window, &canvas);

    const FRAME_WIDTH: usize = BOARD_WIDTH as usize + 2;
    const FRAME_HEIGHT: usize = BOARD_HEIGHT as usize + 2;

    let screen = Rc::new(Screen::new(canvas, FRAME_WIDTH, FRAME_HEIGHT));

    let mut previous_time = window.performance().unwrap().now();
    let mut lag = 0.0;

    let main_loop = Rc::new(RefCell::new(None));
    let main_loop_cont = main_loop.clone();
    {
        let state = state.clone();
        let screen = screen.clone();
        *main_loop.borrow_mut() = Some(Closure::wrap((box move |time: f64| {

            const TIME_STEP: f64 = 500.0;

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
    }
    window.request_animation_frame(
        main_loop.borrow().as_ref().unwrap().as_ref().unchecked_ref()
    ).unwrap();

    add_event_listener(&window, "keyup", move |event: KeyboardEvent| {
        let mut state = state.borrow_mut();
        let key = event.key();
        let new_orientation = match state.segments
            .front().unwrap()
            .orientation
        {
            Orientation::North | Orientation::South => match key.as_ref() {
                "ArrowLeft" => Orientation::West,
                "ArrowRight" => Orientation::East,
                _ => return,
            },
            Orientation::East | Orientation::West => match key.as_ref() {
                "ArrowUp" => Orientation::North,
                "ArrowDown" => Orientation::South,
                _ => return,
            },
        };
        let current_position = state.segments.front().unwrap().start;
        state.segments.push_front(Segment::new(
            current_position,
            0,
            new_orientation,
        ));
    });

    add_event_listener(&window, "resize", move |_: FocusEvent| {
        let window = web_sys::window().unwrap();
        fit_canvas(
            &window,
            window.document().unwrap()
                .get_elements_by_tag_name("canvas")
                .item(0).unwrap()
                .unchecked_ref(),
        );
        screen.resize();
    });
}

fn render(state: &State, screen: &Screen) {
    let context = screen.context();
    context.save();
    context.translate(1.0, 1.0);
    context.save();
    context.translate(0.5, 0.5);
    context.set_line_cap("square");
    for (i, segment) in state.segments.iter().enumerate() {
        context.begin_path();
        context.save();
        context.set_stroke_style(&["green", "red"][i % 2].into());
        context.move_to(segment.start.0, segment.start.1);
        context.line_to(segment.end.0, segment.end.1);
        context.stroke();
        context.restore();
    }
    context.restore();

    context.fill_rect(state.food.0, state.food.1, 1.0, 1.0);
    context.restore();

    context.save();
    context.translate(0.5, 0.5);
    context.stroke_rect(
        0.0, 0.0,
        state.board_size.0 + 1.0,
        state.board_size.1 + 1.0,
    );
    context.restore();

    screen.flip();
}

fn fit_canvas(
    window: &Window,
    canvas: &HtmlCanvasElement,
) {
    let canvas_width = window.inner_width()
        .unwrap().as_f64().unwrap() as i32;
    let canvas_height = window.inner_height()
        .unwrap().as_f64().unwrap() as i32;
    assert!(canvas_width > 0);
    assert!(canvas_height > 0);
    let scale = window.device_pixel_ratio();
    canvas.style().set_property("width", &(canvas_width.to_string() + "px"));
    canvas.style().set_property("height", &(canvas_height.to_string() + "px"));
    let canvas_width = (scale * canvas_width as f64) as usize;
    let canvas_height = (scale * canvas_height as f64) as usize;
    canvas.set_width(canvas_width as u32);
    canvas.set_height(canvas_height as u32);
}

fn add_event_listener<E>(
    window: &Window,
    event_type: &str,
    handler: impl FnMut(E) + 'static,
) where E: FromWasmAbi + 'static {
    let closure: Closure<FnMut(E)> = Closure::new(handler);
    window.add_event_listener_with_callback(
        event_type,
        closure.as_ref().unchecked_ref()
    );
    closure.forget();
}
