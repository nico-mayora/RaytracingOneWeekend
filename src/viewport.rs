// Each time a pixel colour is rendered, send it through a channel to another thread.
// Said thread calls (or is) a function in this very file.
// Each time a colour is received, a matrix is updated (O(1)),
// and the screen is redrawn with this matrix.
// If necessary, add a sleep between screen refreshes.

use super::rtweekend::Colour;
use num::clamp;
use pixels::{Error, Pixels, SurfaceTexture};
use std::sync::mpsc;
use winit::{
    dpi::LogicalSize,
    event::{Event, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};
use winit_input_helper::WinitInputHelper;

#[derive(Debug)]
pub struct ColourPosition {
    pub colour: Colour,
    pub point: (u32, u32),
}

pub struct ViewportRenderer {
    window_width: u32,
    window_height: u32,
    samples_per_pixel: i32,
    window: Window,
    event_loop: EventLoop<()>,
    pixels: Pixels,
}

impl ViewportRenderer {
    pub fn new(window_width: u32, window_height: u32, samples_per_pixel: i32) -> Self {
        let event_loop = EventLoop::new();

        let window = {
            let size = LogicalSize::new(window_width as f64, window_height as f64);
            let scaled_size = LogicalSize::new(window_width as f64 * 2., window_height as f64 * 2.);
            WindowBuilder::new()
                .with_title("Render Result")
                // .with_inner_size(scaled_size)
                .with_min_inner_size(size)
                .build(&event_loop)
                .unwrap()
        };

        let pixels = {
            let window_size = window.inner_size();
            let surface_texture =
                SurfaceTexture::new(window_size.width, window_size.height, &window);
            Pixels::new(window_width, window_height, surface_texture)
        }
        .unwrap();

        Self {
            pixels,
            window_width,
            window_height,
            event_loop,
            samples_per_pixel,
            window,
        }
    }

    pub fn show_rendered_scene(&mut self, receiver: mpsc::Receiver<ColourPosition>) {
        loop {
            let colour_pos = receiver.recv().unwrap();

            let transformed_colour = to_drawn_colour(colour_pos.colour, self.samples_per_pixel);

            let mut frame = self.pixels.frame_mut();
            plot_pixel(&mut frame, colour_pos.point.0 as usize, colour_pos.point.1 as usize, self.window_width as usize, &transformed_colour);
            self.pixels.render().unwrap();
        }
    }
}

fn to_drawn_colour(pixel_colour: Colour, samples_per_pixel: i32) -> [u8; 4] {
    let mut r = pixel_colour[0];
    let mut g = pixel_colour[1];
    let mut b = pixel_colour[2];

    let scale = 1. / samples_per_pixel as f64;
    r = f64::sqrt(r * scale);
    g = f64::sqrt(g * scale);
    b = f64::sqrt(b * scale);

    [
        (256. * clamp(r, 0., 0.999)) as u8,
        (256. * clamp(g, 0., 0.999)) as u8,
        (256. * clamp(b, 0., 0.999)) as u8,
        0xFF,
    ]
}

fn plot_pixel(buffer: &mut [u8], x: usize, y: usize, stride: usize, colour: &[u8]) {
    let i = x + y * stride * 4;

    buffer[i..i + 4].copy_from_slice(colour);
}

unsafe impl Send for ViewportRenderer {}
