use std::{cmp::min, convert::TryInto};
use wasm_bindgen::JsCast;
use web_sys::{
    HtmlCanvasElement,
    OffscreenCanvas,
    CanvasRenderingContext2d,
};

// TODO Decomplect double buffering from scaling?
pub struct Screen {
    front_buffer: ScreenBuffer<HtmlCanvasElement>,
    // TODO Parametrize this?
    back_buffer: ScreenBuffer<OffscreenCanvas>,
    resolution: (usize, usize),
}

// TODO What should this know about resizing?
// TODO Should we protect against/check canvas/context state chaning under us?!
impl Screen {
    pub fn new(canvas: HtmlCanvasElement, width: usize, height: usize) -> Self {
        let screen = Screen {
            front_buffer: ScreenBuffer::new(canvas),
            back_buffer: ScreenBuffer::new(OffscreenCanvas::new(
                // TODO Panic here? Or return a result?
                width.try_into().unwrap(),
                height.try_into().unwrap()
            ).unwrap()),
            resolution: (width, height),
        };
        screen.resize();
        screen
    }

    // TODO Do you really want this?
    pub fn context(&self) -> &CanvasRenderingContext2d {
        &self.back_buffer.context
    }

    // TODO Borrow mutably?

    pub fn resize(&self) {
        // TODO I don't like these explicit casts here!
        // TODO Also protect against this being too large
        //   (to be accurately represented)
        let scale: f64 = min(
            self.front_buffer.canvas.width() as usize / self.resolution.0,
            self.front_buffer.canvas.height() as usize / self.resolution.1,
        ) as f64;
        // TODO Make sure this is not 0?
        // TODO This assumes that the context was actually reset by a resize ...
        self.front_buffer.context.set_image_smoothing_enabled(false);
        self.front_buffer.context.scale(
            scale as f64,
            scale as f64,
        );
    }

    pub fn flip(&self) {
        self.front_buffer.context.clear_rect(
            0.0,
            0.0,
            self.front_buffer.canvas.width().into(),
            self.front_buffer.canvas.height().into(),
        );
        self.front_buffer.context.draw_image_with_image_bitmap(
            // TODO Do we need to `close` this?
            &self.back_buffer.canvas.transfer_to_image_bitmap().unwrap(),
            // TODO Center this?
            0.0,
            0.0
        ).unwrap();
    }
}

struct ScreenBuffer<Canvas: Draw2d> {
    canvas: Canvas,
    context: CanvasRenderingContext2d,
}

impl<Canvas: Draw2d> ScreenBuffer<Canvas> {
    fn new(canvas: Canvas) -> Self {
        let context = canvas.get_context_2d();
        ScreenBuffer {
            canvas: canvas,
            context: context,
        }
    }
}

trait Draw2d {
    fn get_context_2d(&self) -> CanvasRenderingContext2d;
}

macro_rules! draw2d {
    ($t:ty) => {
        impl Draw2d for $t {
            // TODO Return a `Result`?
            fn get_context_2d(&self) -> CanvasRenderingContext2d {
                self.get_context("2d")
                    .unwrap()
                    .unwrap()
                    .unchecked_into()
            }
        }
    };
}

draw2d!(HtmlCanvasElement);
draw2d!(OffscreenCanvas);
