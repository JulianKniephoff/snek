#![feature(box_syntax)]
#![feature(try_from)]

extern crate rand;
extern crate wasm_bindgen;
extern crate web_sys;
extern crate console_error_panic_hook;

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
    KeyboardEvent,
    FocusEvent,
};
use screen::{Screen};

// TODO Name this `Snek`?!
struct State {
    board_size: (f64, f64),
    segments: VecDeque<Segment>,
    food: (f64, f64),
    // TODO These should not be a `Vec`
    // TODO Especially this first one should just be a slice!
    occupied: Vec<bool>,
    // TODO We save this mostly to avoid reallocating it every food spawn ...
    //   The compiler shoould optimize this but I don't see how it could.
    free_cells: Vec<usize>,
    had_food: bool,
    // TODO Should this really be in the state?
    //   And should the input handling be in `update` for that matter?
    new_direction: Option<(f64, f64)>,
    paused: bool,
}

// TODO Maybe create some kind of `Simulation` trait
//   that would also dictate the time step.
//   Would you want to allow for variable time step in that??!?

// TODO Do we really want to store stuff as floats?
//   Especially the size?!
//   Maybe the state should already work in discretized space ...
impl State {
    fn new(board_width: f64, board_height: f64) -> Self {
        assert!(board_width >= 0.0);
        assert!(board_height >= 0.0);
        const starting_length: usize = 5;
        assert!(board_width > starting_length as f64);
        // TODO Should this really be a float?
        // TODO Position this somewhat sensibly
        // TODO Infer this position from the segment somehow
        //   Maybe give it a function to compute its head position?
        let starting_position = ((starting_length - 1) as f64, 0.0);
        let cell_count = board_width as usize * board_height as usize;
        let mut occupied = vec![false; cell_count];
        // TODO Make this more dynamic/dependent on the starting configuration ...
        //   Because this **will** break!
        for x in 0..starting_length {
            occupied[x] = true;
        }
        let mut free_cells = Vec::with_capacity(cell_count);
        State {
            board_size: (board_width, board_height),
            food: State::spawn_food(
                // TODO You construct this tuple pretty often ...
                (board_width as usize, board_height as usize),
                &occupied,
                // TODO That we have to pass the buffer here is utter madness
                &mut free_cells,
            ),
            // TODO Is there no `vec!`-like macro for this?
            segments: {
                let mut segments = VecDeque::new();
                segments.push_back(Segment::new(
                    starting_position,
                    // TODO Because of this, our coordinates can now no longer be unsigned ...
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

    // TODO Getting this in the right order was a bitch
    //   Can you simplify this using the "Elm pattern"?
    // TODO Note that this probably horribly breaks
    //   when the Snek is only one pixel long ...
    fn update(&mut self) {
        if self.paused {
            return;
        }

        // TODO That all this is wrapped in a black to get the current head,
        //   just to appease the borrow checker,
        //   is a kind of madness
        let head_start = if let Some(new_direction) = self.new_direction.take() {
            let current_head = self.segments.front().unwrap();
            let new_start = (
                current_head.start.0 + new_direction.0,
                current_head.start.1 + new_direction.1,
            );
            self.segments.push_front(Segment::new(
                new_start,
                current_head.start,
            ));
            new_start
        } else {
            let head = self.segments.front_mut().unwrap();
            // TODO Declarations like these are now unnecessary
            let direction = head.direction;

            // TODO I don't like how we have similar calculations like these
            //   in both branches.
            head.start.0 += direction.0;
            head.start.1 += direction.1;
            head.start
        };

        if head_start.0 < 0.0
            || head_start.0 >= self.board_size.0
            || head_start.1 < 0.0
            || head_start.1 >= self.board_size.1
        {
            return self.game_over();
        }

        // TODO Maybe defer elongation even more?
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
            // TODO I don't like that we do this twice,
            //   but the order requirements of this function
            //   demands it.
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

    // TODO It sucks that this is an associated function
    //   only because we use it before the `State` is fully constructed
    // TODO Should these really be associated functions at all, for that matter?
    //   They could just be free functions,
    //   especially when this code moves to a dedicated module.
    // TODO What type should we take here?
    fn spawn_food<'a>(
        // TODO Actually we only need the width ...
        board_size: (usize, usize),
        // TODO I don't like the `&bool`
        occupied: impl IntoIterator<Item = &'a bool>,
        free_cells: &mut Vec<usize>,
    ) -> (f64, f64) {
        free_cells.clear();
        free_cells.extend(
            occupied.into_iter()
                .enumerate()
                // TODO I don't like the `&` here
                .filter(|(_, &occupied)| !occupied)
                .map(|(index, _)| index)
        );
        // TODO Is `thread_rng` the right way?
        //   How else should we get this?
        // TODO If you do this differently,
        //   remember to get rid of the `wasm-bindgen` feature
        //   in `Cargo.toml`
        // TODO Seed this?
        // TODO Make sure that this is really never called with a full field
        State::to_position(
            *free_cells.choose(&mut thread_rng()).unwrap(),
            (board_size.0 as f64, board_size.1 as f64),
        )
    }

    // TODO We only need the width ...
    // TODO Also at this point it seems ridiculous to save the size as floats
    // TODO That this is a member and `to_position` an associated function is madness
    // TODO Maybe both should be members of some kind of `Board` structure?
    // TODO Abstract coordinates, etc.
    //   But I mean having a function like this already suggests that your positions are discrete ...
    fn to_position(index: usize, size: (f64, f64)) -> (f64, f64) {

        // TODO Why is there no divmod?!
        ((index % size.0 as usize) as f64, (index / size.0 as usize) as f64)
    }

    // TODO That this is a member while the above is static is madness
    fn to_index(&self, position: (f64, f64)) -> usize {
        (position.1 * self.board_size.0 + position.0) as usize
    }

    fn occupy(&mut self, position: (f64, f64), occupied: bool) {
        let index = self.to_index(position);
        self.occupied[index] = occupied;
    }
}

struct Segment {
    // TODO Create a newtype for coordinates?
    //   That does bounds checking??!?
    start: (f64, f64),
    behind: (f64, f64),
    direction: (f64, f64),
}

impl Segment {
    // TODO Note that this is now potentially unsafe ...
    //   Can we add appropriate checks?
    //   Look at the assembly to see what this would cost us
    // TODO Create situationally more appropriate versions of this?
    //   Taking length and orientation maybe?
    fn new(start: (f64, f64), behind: (f64, f64)) -> Self {
        // TODO Note that everything might break if you compare to `0.0`,
        //   because of `-0.0`.
        //   This happens in multiple places in the code!!!
        Segment {
            start,
            behind,
            direction: (
                // TODO Oh my god, make these things integers already
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

// TODO Actually make this work with a provided canvas, too?
fn snek() {
    console_error_panic_hook::set_once();

    // TODO Do you really want constants here?
    //   Well, I want to initialize the `FRAME_*` constants with these
    // TODO Encode the invariant that this is screen size - 2 somehow
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

    // To create a screen buffer:
    /*
    let mut frame_buffer = Vec::with_capacity(
        FRAME_WIDTH.checked_mul(FRAME_HEIGHT)
            .and_then(|size| size.checked_mul(4))
            .unwrap()
    );
    */

    let screen = Rc::new(Screen::new(canvas, FRAME_WIDTH, FRAME_HEIGHT));

    let mut previous_time = window.performance().unwrap().now();
    let mut lag = 0.0;

    // TODO Wrap this in a cool function
    // TODO Use `Closure::new`
    // TODO How to pass proper functions?
    let main_loop = Rc::new(RefCell::new(None));
    let main_loop_cont = Rc::clone(&main_loop);
    {
        let state = Rc::clone(&state);
        let screen = Rc::clone(&screen);
        // TODO Note `time` is actually the start of the frame ...
        *main_loop.borrow_mut() = Some(Closure::wrap((box move |time: f64| {

            // TODO More sophisticated structure?
            //   But then we need to throttle it ...

            // TODO Where exactly do you want to convert to seconds,
            //   if at all
            const TIME_STEP: f64 = 500.0;

            // TODO Naming
            lag += time - previous_time;
            previous_time = time;

            // TODO Fix the start of the simulation
            //   You call `update` before you render the first time.
            //   Also the first frame is going to be shorter?
            if lag >= TIME_STEP {
                // TODO Maybe make this produce a temporary state?
                //   So that you can pass the remaining time in here as well,
                //   to get to the inbetween frames?
                // TODO Also pass all of time passed?
                // TODO Should we even pass the step?
                //   In a way, `update` should determine
                //   how small or big it can be.
                //   This also means that the constant
                //   should live somewhere else.
                state.borrow_mut().update();
                lag = 0.0;

                render(&state.borrow(), &screen);
            }

            // TODO Store the window outside the closure?
            web_sys::window().unwrap().request_animation_frame(
                (
                    main_loop_cont
                        .borrow()
                        .as_ref()
                        // TODO Can we somehow get rid of this?
                        .unwrap() as &Closure<_>
                )
                    .as_ref()
                    .unchecked_ref()
            ).unwrap();
        // TODO Why not just call `main_loop` here?!
        //   This is actually harder than it seems
        //   since we move the function into the closure.
        }) as Box<dyn FnMut(_)>));
    }
    window.request_animation_frame(
        // TODO Factor this out ...
        //   Maybe this would get rid of the type annotation as well?
        main_loop.borrow().as_ref().unwrap().as_ref().unchecked_ref()
    ).unwrap();

    // TODO How can we remove these listeners again?
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

        // TODO Make this nicer ...
        //   Maybe factor out some `is_orthogonal`,
        //   then just map a key to a new orientation,
        //   Profit??!?
        // TODO Why can't you do something like this?
        /*let new_orientation = match (
            event.key().as_ref(),
            state.borrow().segments.front().unwrap().orientation,
        ) {
            ("ArrowLeft", Orientation::North | Orientation::South) =>
                Orientation::West,
            ("ArrowRight", Orientation::North | Orientation::South) =>
                Orientation::East,
            ("ArrowUp", Orientation::East | Orientation::West) =>
                Orientation::North,
            ("ArrowDown", Orientation::East | Orientation::West) =>
                Orientation::South,
            _ => return,
        };*/
        // TODO This is now less readable,
        //   without `Orientation`
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

    // TODO Throttle!!!
    // TODO Can you somehow pass a function without argument?
    //   A second function? Overlaoding using traits??!
    // TODO Can you just pass `Event`?
    // TODO Why does this take a `FocusEvent`??!??
    add_event_listener(&window, "resize", move |_: FocusEvent| {
        let window = web_sys::window().unwrap();
        fit_canvas(
            &window,
            // TODO Make the canvas shared as well?!
            //   But at least make it safely identifyable ...
            window.document().unwrap()
                .get_elements_by_tag_name("canvas")
                .item(0).unwrap()
                .unchecked_ref(),
        );
        screen.resize();
    });
}

// TODO This should be factored differently
fn render(state: &State, screen: &Screen) {
    if state.paused {
        // TODO Should flipping be taken care of by some kind of container?
        screen.flip();
        return;
    }

    // TODO Does none of this warn?!
    let context = screen.context();
    // TODO Woff, this "transformer stack" seems ugly
    //   Also it might need comments
    //   Or self documenting functions ...
    // TODO Set this somewhere else?!
    context.save();
    context.translate(1.0, 1.0);
    context.save();
    context.translate(0.5, 0.5);
    context.set_line_cap("square");
    // TODO Use `reduce` somehow?
    for (i, segment) in state.segments.iter().enumerate() {
        context.begin_path();
        context.save();
        // TODO Use `zip` or something
        // TODO This is in the loop at the moment
        //   mostly because of the segment coloring
        //   Simplify this path handling once you get rid of it
        context.set_stroke_style(&["green", "red"][i % 2].into());
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
    // TODO Just add to the coordinates manually??!?
    // TODO Can we simplify the transformation stuff?
    //   It sucks that different parts of this method
    //   need different, overlapping transformations ...
    context.translate(0.5, 0.5);
    context.stroke_rect(
        0.0, 0.0,
        state.board_size.0 + 1.0,
        state.board_size.1 + 1.0,
    );
    context.restore();

    screen.flip();

    /* TODO Get rid of this and all the stuff that it needs?
      Also the feature(s) in `Cargo.toml`
    frame_buffer.clear();
    // TODO Use more efficient indexing?
    for y in 0..frame_height {
        for x in 0..frame_width {
            let pixel = (
                ((x + y) % 2) as f64
                    * ((state.time_passed / 1000.0).sin() + 1.0) / 2.0
                    * u8::max_value() as f64
            ) as u8;
            frame_buffer.push(pixel);
            frame_buffer.push(pixel);
            frame_buffer.push(pixel);
            frame_buffer.push(u8::max_value());
        }
    }

    back_context.put_image_data(
        &ImageData::new_with_u8_clamped_array_and_sh(
            Clamped(frame_buffer),
            frame_width as u32,
            frame_height as u32,
        ).unwrap(),
        // TODO Center this
        //   Although it might be simpler to just center the canvas?
        //   Horizontally maybe, but vertically?
        //   You would probably need CSS for that.
        0.0,
        0.0,
    ).unwrap();
    */
}

fn fit_canvas(
    window: &Window,
    canvas: &HtmlCanvasElement,
) {
    // TODO Exclude scrollbars?
    let canvas_width = window.inner_width()
        .unwrap().as_f64().unwrap() as i32;
    let canvas_height = window.inner_height()
        .unwrap().as_f64().unwrap() as i32;
    assert!(canvas_width > 0);
    assert!(canvas_height > 0);
    let scale = window.device_pixel_ratio();
    // TODO Should this really be necessary?
    // TODO We might be able to circumvent the bluriness when zoomed in
    //   more easily using the `image-rendering` CSS property
    canvas.style().set_property("width", &(canvas_width.to_string() + "px"));
    // TODO Actually setting the height is redundant ...
    canvas.style().set_property("height", &(canvas_height.to_string() + "px"));
    let canvas_width = (scale * canvas_width as f64) as usize;
    let canvas_height = (scale * canvas_height as f64) as usize;
    canvas.set_width(canvas_width as u32);
    canvas.set_height(canvas_height as u32);
}

// TODO Can you somehow import this function directly with `wasm_bindgen`?
// TODO Make this more general?
//   Allow functions without argument?
//   Allow return types?
//   Factor out the actual registration code as well?
//   Then you need to be able to take even more different closures ...
//   Can a macro help?!
// TODO Is `FnMut` the right bound?
// TODO You now lose the optimization of `box`-ing the closure directly ...
fn add_event_listener<E>(
    window: &Window,
    event_type: &str,
    handler: impl FnMut(E) + 'static,
    // TODO Why can this `where` not be written using `impl` above?!
    //   Maybe you want to put all bounds in the `where` then?
    // TODO Why does `E` have to be `static`?!
    // TODO Do we really need this bound?
) where E: FromWasmAbi + 'static {
    // TODO Why do we need type annotations here?!
    // TODO Can we use `wrap` instead? Should we?
    let closure: Closure<FnMut(E)> = Closure::new(handler);
    window.add_event_listener_with_callback(
        event_type,
        closure.as_ref().unchecked_ref()
    );
    // TODO Unwrap?
    closure.forget();
}
