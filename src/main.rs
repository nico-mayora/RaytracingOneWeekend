use raytracing::rtweekend::Colour;
use raytracing::raytracer::*;
use num::clamp;
use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event::VirtualKeyCode,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};
use winit_input_helper::WinitInputHelper;
use crossbeam::channel::*;
// use core::sync;
use std::thread;
use config::Config;

pub struct ViewportData {
    window: Window,
    pixels: Pixels,
    event_loop: EventLoop<()>,
    input: WinitInputHelper,
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

fn show_rendered_scene(
    window_width: u32,
    window_height: u32,
    samples_per_pixel: i32,
    pixel_batch_size: usize,
    win_scale: f64,
    receiver: Receiver<ColourPosition>,
    sync_recv: Receiver<()>,
) {
    let viewport_data = initialise_viewport(window_width, window_height, win_scale);

    let event_loop = viewport_data.event_loop;
    let mut input = viewport_data.input;
    let mut pixels = viewport_data.pixels;
    let window = viewport_data.window;

    let mut colour_buffer: Vec<ColourPosition> = Vec::new();
    let mut channel_active = true;

    event_loop.run(move |event, _, control_flow| {
        while channel_active && colour_buffer.len() < pixel_batch_size {
            // rendering done. reset channel_active and drain receiver into colour_buffer.
            if let Ok(()) = sync_recv.try_recv() {
                channel_active = false;
                while let Ok(cp) = receiver.try_recv() {
                    colour_buffer.push(cp);
                }
                break;
            }

            match receiver.try_recv() {
                Ok(colour_pos) => {
                    colour_buffer.push(colour_pos);
                }
                _ => { break; }
            }
        }

        draw(&mut pixels, &colour_buffer, samples_per_pixel, window_height, window_width);
        colour_buffer.clear();

        // Handle input events
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.close_requested() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            if input.key_pressed(VirtualKeyCode::Escape) || input.close_requested() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                if pixels.resize_surface(size.width, size.height).is_err() {
                    *control_flow = ControlFlow::Exit;
                    return;
                }
            }

        }
        window.request_redraw();
    });
}

fn initialise_viewport(
    window_width: u32,
    window_height: u32,
    window_scale: f64,
) -> ViewportData {
    let event_loop = EventLoop::new();

    let input = WinitInputHelper::new();

    let window = {
        let size = LogicalSize::new(window_width as f64, window_height as f64);
        let scaled_size = LogicalSize::new(window_scale * window_width as f64, window_scale * window_height as f64);
        WindowBuilder::new()
            .with_title("Render Result")
            .with_inner_size(size)
            .with_min_inner_size(scaled_size)
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

fn draw(
    pixels: &mut Pixels,
    colour_buffer: &Vec<ColourPosition>,
    samples_per_pixel: i32,
    window_height: u32,
    window_width: u32,
) {
    let frame = pixels.frame_mut();

    for cp in colour_buffer {
        let transformed_colour = to_drawn_colour(cp.colour, samples_per_pixel);
        plot_pixel(
            frame,
            cp.point.0 as usize,
            cp.point.1 as usize,
            &transformed_colour,
            window_width,
            window_height,
            );
    }

    pixels.render().unwrap();
}

fn get_settings() -> Config {
    Config::builder()
        .add_source(config::File::with_name("config.toml"))
        .build()
        .unwrap()
}

fn main() {
    let settings = get_settings();
    let rt_settings = settings.clone();

    let (sender, receiver) = unbounded::<ColourPosition>();
    let (sync_snd, sync_recv) = unbounded::<()>();
    let _raytracer_handle = thread::spawn(move || render(sender, sync_snd, rt_settings));

    let win_width = settings.get::<u32>("image.width").unwrap();
    let win_height = {
        let aspect_ratio = settings.get_float("camera.aspect_ratio").unwrap();
        let wh = (win_width as f64 / aspect_ratio) as u32;
        dbg!(wh, win_width);
        wh
    };
    let samples_per_pixel = settings.get::<i32>("image.samples_per_pixel").unwrap();
    let pixel_batch_size = settings.get::<usize>("viewport.pixel_batch_size").unwrap();
    let window_scale = settings.get::<f64>("viewport.window_scale").unwrap();

    show_rendered_scene(win_width, win_height, samples_per_pixel, pixel_batch_size, window_scale, receiver, sync_recv);
}
