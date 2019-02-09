use std::{cmp::min, convert::TryInto};
use wasm_bindgen::JsCast;
use web_sys::{
    HtmlCanvasElement,
    OffscreenCanvas,
    CanvasRenderingContext2d,
};

pub struct Screen {
    front_buffer: ScreenBuffer<HtmlCanvasElement>,
    back_buffer: ScreenBuffer<OffscreenCanvas>,
    resolution: (usize, usize),
}

impl Screen {
    pub fn new(canvas: HtmlCanvasElement, width: usize, height: usize) -> Self {
        let screen = Screen {
            front_buffer: ScreenBuffer::new(canvas),
            back_buffer: ScreenBuffer::new(OffscreenCanvas::new(
                width.try_into().unwrap(),
                height.try_into().unwrap()
            ).unwrap()),
            resolution: (width, height),
        };
        screen.resize();
        //screen.back_buffer.context.set_image_smoothing_enabled(false);
        screen
    }

    pub fn context(&self) -> &CanvasRenderingContext2d {
        &self.back_buffer.context
    }

    pub fn resize(&self) {
        let scale: f64 = min(
            self.front_buffer.canvas.width() as usize / self.resolution.0,
            self.front_buffer.canvas.height() as usize / self.resolution.1,
        ) as f64;
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
            &self.back_buffer.canvas.transfer_to_image_bitmap().unwrap(),
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
