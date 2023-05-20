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

fn plot_pixel(buffer: &mut [u8], x: usize, y: usize, colour: &[u8], window_width: u32, window_height: u32) {
    let y = (window_height - 1 - y as u32) as usize; // unflip
    let i = x + y * window_width as usize * 4;

    buffer[i..i + 4].copy_from_slice(colour);
}

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
            WindowBuilder::new()
                .with_title("Render Result")
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
        let mut colour_buffer: Vec<ColourPosition> = Vec::new();
        loop {
            let colour_pos = receiver.recv().unwrap();
            colour_buffer.push(colour_pos);

            // drawing pixels one by one takes too long
            if colour_buffer.len() < 5000 {
                continue;
            }

            let mut frame = self.pixels.frame_mut();
            for cp in &colour_buffer {
                let transformed_colour = to_drawn_colour(cp.colour, self.samples_per_pixel);
                plot_pixel(
                    &mut frame,
                    cp.point.0 as usize,
                    cp.point.1 as usize,
                    &transformed_colour,
                    self.window_width,
                    self.window_height
                );
            }

            colour_buffer.clear();
            self.pixels.render().unwrap();
        }
    }
}

unsafe impl Send for ViewportRenderer {}
