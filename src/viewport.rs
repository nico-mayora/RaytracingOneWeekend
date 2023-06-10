// Implement ""eventually"" the logic for handling events and the window properly :)
// Just tried. It is *not* trivial to do so.

use super::rtweekend::Colour;
use num::clamp;
use pixels::{Pixels, SurfaceTexture};
use std::sync::mpsc::{Receiver, TryRecvError};
use winit::{
    dpi::LogicalSize,
    event::{Event, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};
use winit_input_helper::WinitInputHelper;

const PIXEL_BATCH_SIZE: usize = 10_000;
// const WINDOW_SCALE: f64 = 0.1;

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

fn plot_pixel(
    buffer: &mut [u8],
    x: usize,
    y: usize,
    colour: &[u8],
    window_width: u32,
    window_height: u32,
) {
    let y = (window_height - 1 - y as u32) as usize; // unflip
    let i = (x + y * window_width as usize) * 4;

    buffer[i..i + 4].copy_from_slice(colour);
}

#[derive(Debug)]
pub struct ColourPosition {
    pub colour: Colour,
    pub point: (u32, u32),
}

pub struct ViewportData {
    window: Window,
    pixels: Pixels,
    event_loop: EventLoop<()>,
    input: WinitInputHelper,
}

pub fn show_rendered_scene(
    window_width: u32,
    window_height: u32,
    samples_per_pixel: i32,
    receiver: Receiver<ColourPosition>,
) {
    let viewport_data = initialise_viewport(window_width, window_height);

    let event_loop = viewport_data.event_loop;
    let mut input = viewport_data.input;
    let mut pixels = viewport_data.pixels;
    let window = viewport_data.window;

    let mut colour_buffer: Vec<ColourPosition> = Vec::new();
    let mut channel_active = true;

    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            let mut received = 0;
            while channel_active && received < PIXEL_BATCH_SIZE {
                dbg!("jamon");
                match receiver.try_recv() {
                    Ok(colour_pos) => {
                        colour_buffer.push(colour_pos);
                        received += 1;
                    }
                    Err(TryRecvError::Empty) => {
                        draw(&mut pixels, &colour_buffer, samples_per_pixel, window_height, window_width);
                        colour_buffer.clear();
                        break;
                    }
                    Err(TryRecvError::Disconnected) => {
                        channel_active = false;
                    }
                }
            }
        }

        // Handle input events
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.close_requested() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                if let Err(_) = pixels.resize_surface(size.width, size.height) {
                    *control_flow = ControlFlow::Exit;
                    return;
                }
            }

            window.request_redraw();
        }
    });
}

fn initialise_viewport(
    window_width: u32,
    window_height: u32,
) -> ViewportData {
    let event_loop = EventLoop::new();

    let input = WinitInputHelper::new();

    let window = {
        let size = LogicalSize::new(window_width as f64, window_height as f64);
        WindowBuilder::new()
            .with_title("Render Result")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(window_width, window_height, surface_texture)
    }
    .unwrap();

    ViewportData {
        pixels,
        window,
        event_loop,
        input,
    }
}

pub fn draw(
    pixels: &mut Pixels,
    colour_buffer: &Vec<ColourPosition>,
    samples_per_pixel: i32,
    window_width: u32,
    window_height: u32,
) {
    let mut frame = pixels.frame_mut();

    for cp in colour_buffer {
        let transformed_colour = to_drawn_colour(cp.colour, samples_per_pixel);
        plot_pixel(
            &mut frame,
            cp.point.0 as usize,
            cp.point.1 as usize,
            &transformed_colour,
            window_width,
            window_height,
            );
    }


    println!("before");
    pixels.render().unwrap();
    println!("after");
}
