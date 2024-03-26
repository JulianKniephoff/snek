mod screen;

use std::{cell::RefCell, rc::Rc, collections::VecDeque};
use rand::{thread_rng, seq::SliceRandom};
use wasm_bindgen::{
    prelude::wasm_bindgen,
    JsCast,
    closure::Closure,
    convert::FromWasmAbi,
};
use web_sys::{
    HtmlCanvasElement,
    Window,
    EventTarget,
    KeyboardEvent,
    FocusEvent,
};
use screen::Screen;

struct State {
    board_size: (f64, f64),
    segments: VecDeque<Segment>,
    food: (f64, f64),
    occupied: Vec<bool>,
    free_cells: Vec<usize>,
    had_food: bool,
    new_direction: Option<(f64, f64)>,
    paused: bool,
}

impl State {
    fn new(board_width: f64, board_height: f64) -> Self {
        assert!(board_width >= 0.0);
        assert!(board_height >= 0.0);
        const STARTING_LENGTH: usize = 5;
        assert!(board_width > STARTING_LENGTH as f64);
        const STARTING_POSITION: (f64, f64) = ((STARTING_LENGTH - 1) as f64, 0.0);
        let cell_count = board_width as usize * board_height as usize;
        let mut occupied = vec![false; cell_count];
        for x in 0..STARTING_LENGTH {
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
            segments: {
                let mut segments = VecDeque::new();
                segments.push_back(Segment::new(
                    STARTING_POSITION,
                    (-1.0, 0.0),
                ));
                segments
            },
            occupied,
            free_cells,
            had_food: false,
            new_direction: None,
            paused: false,
        }
    }

    fn update(&mut self) {
        if self.paused {
            return;
        }

        let head_start = if let Some(new_direction) = self.new_direction.take() {
            let current_head_start = self.segments.front().unwrap().start;
            let new_start = (
                current_head_start.0 + new_direction.0,
                current_head_start.1 + new_direction.1,
            );
            self.segments.push_front(Segment::new(
                new_start,
                current_head_start,
            ));
            new_start
        } else {
            let head = self.segments.front_mut().unwrap();
            head.start.0 += head.direction.0;
            head.start.1 += head.direction.1;
            head.start
        };

        if head_start.0 < 0.0
            || head_start.0 >= self.board_size.0
            || head_start.1 < 0.0
            || head_start.1 >= self.board_size.1
        {
            return self.game_over();
        }

        if self.had_food {
            self.had_food = false;
        } else {
            let tail = self.segments.back_mut().unwrap();
            tail.behind.0 += tail.direction.0;
            tail.behind.1 += tail.direction.1;
            let tail_start = tail.start;
            let tail_behind = tail.behind;
            self.occupy(tail_behind, false);
            if tail_start == tail_behind {
                self.segments.pop_back();
            }
        }

        if self.occupied[self.to_index(head_start)] {
            return self.game_over();
        }

        self.occupy(head_start, true);

        if head_start == self.food {
            self.food = State::spawn_food(
                (self.board_size.0 as usize, self.board_size.1 as usize),
                &self.occupied,
                &mut self.free_cells,
            );
            self.had_food = true;
        }
    }

    fn game_over(&mut self) {
        *self = State::new(self.board_size.0, self.board_size.1);
    }

    fn spawn_food<'a>(
        board_size: (usize, usize),
        occupied: impl IntoIterator<Item = &'a bool>,
        free_cells: &mut Vec<usize>,
    ) -> (f64, f64) {
        free_cells.clear();
        free_cells.extend(
            occupied.into_iter()
                .enumerate()
                .filter(|(_, &occupied)| !occupied)
                .map(|(index, _)| index)
        );
        State::to_position(
            *free_cells.choose(&mut thread_rng()).unwrap(),
            (board_size.0 as f64, board_size.1 as f64),
        )
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
    start: (f64, f64),
    behind: (f64, f64),
    direction: (f64, f64),
}

impl Segment {
    fn new(start: (f64, f64), behind: (f64, f64)) -> Self {
        Segment {
            start,
            behind,
            direction: (
                ((start.0 - behind.0) as isize).signum() as f64,
                ((start.1 - behind.1) as isize).signum() as f64,
            )
        }
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    snek();
}

const PIXELS_PER_TILE: usize = 2;

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

    const FRAME_WIDTH: usize = PIXELS_PER_TILE * (BOARD_WIDTH as usize + 2);
    const FRAME_HEIGHT: usize = PIXELS_PER_TILE * (BOARD_HEIGHT as usize + 2);

    let screen = Rc::new(Screen::new(canvas, FRAME_WIDTH, FRAME_HEIGHT));

    let mut previous_time = window.performance().unwrap().now();
    let mut lag = 0.0;

    let main_loop = Rc::new(RefCell::new(None));
    let main_loop_cont = Rc::clone(&main_loop);
    {
        let state = Rc::clone(&state);
        let screen = Rc::clone(&screen);
        *main_loop.borrow_mut() = Some(Closure::wrap(Box::new(move |time: f64| {

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

        if key == " " {
            state.paused = !state.paused;
            return;
        }

        if state.paused {
            return;
        }

        if state.new_direction.is_some() {
            return;
        }
        let direction = state.segments
            .front().unwrap()
            .direction;
        state.new_direction = Some(if direction.0 == 0.0 {
            // We are currently moving vertically
            match key.as_ref() {
                "ArrowLeft" => (-1.0, 0.0),
                "ArrowRight" => (1.0, 0.0),
                _ => return,
            }
        } else if direction.1 == 0.0 {
            // We are currently moving horizontally
            match key.as_ref() {
                "ArrowUp" => (0.0, -1.0),
                "ArrowDown" => (0.0, 1.0),
                _ => return,
            }
        } else {
            return
        });
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
    if state.paused {
        screen.flip();
        return;
    }

    let context = screen.context();

    context.save();
    context.scale(PIXELS_PER_TILE as f64, PIXELS_PER_TILE as f64).unwrap();

    context.save();
    context.translate(1.0, 1.0).unwrap();
    context.save();
    context.translate(0.5, 0.5).unwrap();
    context.set_line_cap("square");
    for segment in &state.segments {
        context.begin_path();
        context.save();
        context.move_to(segment.start.0, segment.start.1);
        context.line_to(
            segment.behind.0 + segment.direction.0,
            segment.behind.1 + segment.direction.1,
        );
        context.stroke();
        context.restore();
    }
    context.restore();

    context.fill_rect(state.food.0, state.food.1, 1.0, 1.0);
    context.restore();

    context.save();
    context.translate(0.5, 0.5).unwrap();
    context.stroke_rect(
        0.0, 0.0,
        state.board_size.0 + 1.0,
        state.board_size.1 + 1.0,
    );
    context.restore();

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
    canvas.style().set_property("width", &(canvas_width.to_string() + "px")).unwrap();
    canvas.style().set_property("height", &(canvas_height.to_string() + "px")).unwrap();
    let canvas_width = (scale * canvas_width as f64) as usize;
    let canvas_height = (scale * canvas_height as f64) as usize;
    canvas.set_width(canvas_width as u32);
    canvas.set_height(canvas_height as u32);
}

fn add_event_listener<E>(
    target: &EventTarget,
    event_type: &str,
    handler: impl FnMut(E) + 'static,
) where E: FromWasmAbi + 'static {
    let closure: Closure<dyn FnMut(E)> = Closure::new(handler);
    target.add_event_listener_with_callback(
        event_type,
        closure.as_ref().unchecked_ref()
    ).unwrap();
    closure.forget();
}
