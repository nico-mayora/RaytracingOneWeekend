// use crossbeam::channel::*;
// use pixels::{Pixels, SurfaceTexture};
// use winit::application::ApplicationHandler;
// use winit::event::WindowEvent;
// use winit::event_loop::ActiveEventLoop;
// use winit::window::{WindowId, WindowAttributes};
// use winit::{
//     dpi::{LogicalSize, Size},
//     event_loop::{ControlFlow, EventLoop},
//     window::Window,
// };

// // FIXME: read from config.toml
// const PIXEL_BATCH_SIZE: i32 = 100_000;
// const SAMPLES_PER_PIXEL: i32 = 100;
// const WINDOW_HEIGHT: u32 = 800;
// const WINDOW_WIDTH: u32 = 400;
// const WINDOW_SCALE: f64 = 1.5;
// const WINDOW_TITLE: String = "Raytracer".into();

// use crate::colour::drawn_colour;
// use crate::raytracer::ColourPosition;

// pub struct App {
//     window: Window,
//     pixels: Pixels,
//     channel_active: bool,
//     receiver: Receiver<ColourPosition>,
//     sync_recv: Receiver<()>,
//     colour_buffer: Vec<ColourPosition>,
// }

// impl App {
//     fn new(&mut self, event_loop: &ActiveEventLoop) -> Self {
//         let size = Size::Logical(LogicalSize {
//             width: WINDOW_WIDTH.into(),
//             height: WINDOW_HEIGHT.into()
//         });
//         let window_attributes = WindowAttributes {
//             inner_size: Some(size),
//             resizable: false,
//             title: WINDOW_TITLE,
//             visible: true,
//             transparent: false,
//             ..Default::default()
//         };

//         let window = event_loop.create_window(window_attributes).unwrap();

//         let pixels = {
//             let window_size = window.inner_size();
//             let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
//             Pixels::new(window_width, window_height, surface_texture)
//         }
//         .unwrap();

//         Self {
//             window,
//         }
            

        
        
//     }
// }

// impl ApplicationHandler for App {
//     fn resumed(&mut self, event_loop: &ActiveEventLoop) {
//         // TODO: Check if this is needed.
//         self.window = event_loop.create_window(Window::default_attributes()).unwrap();
//     }

//     fn window_event(
//         &mut self,
//         event_loop: &ActiveEventLoop,
//         window_id: WindowId,
//         event: WindowEvent,
//     ) {
//         let colour_buffer =
//         match event {
//             WindowEvent::CloseRequested => {
//                 println!("Closing...");
//                 event_loop.exit();
//             }
//             WindowEvent::RedrawRequested => {
//                 // TODO: make pixel_batch_size static
//                 while self.channel_active && self.colour_buffer.len() < PIXEL_BATCH_SIZE as usize {
//                     // rendering done. reset channel_active and drain receiver into colour_buffer.
//                     if let Ok(()) = self.sync_recv.try_recv() {
//                         self.channel_active = false;
//                         while let Ok(cp) = self.receiver.try_recv() {
//                             self.colour_buffer.push(cp);
//                         }
//                         break;
//                     }

//                     match self.receiver.try_recv() {
//                         Ok(colour_pos) => {
//                             self.colour_buffer.push(colour_pos);
//                         }
//                         _ => {
//                             break;
//                         }
//                     }
//                 }

//                 draw(
//                     &mut self.pixels,
//                     &self.colour_buffer,
//                     SAMPLES_PER_PIXEL,
//                     WINDOW_HEIGHT,
//                     WINDOW_WIDTH,
//                 );
//                 self.colour_buffer.clear();
//                 self.window.request_redraw();
//             }
//             _ => (),
//         };
//     }
// }

// fn plot_pixel(
//     buffer: &mut [u8],
//     x: usize,
//     y: usize,
//     colour: &[u8],
//     window_width: u32,
//     window_height: u32,
// ) {
//     let y = (window_height - 1 - y as u32) as usize; // unflip
//     let i = (x + y * window_width as usize) * 4;

//     buffer[i..i + 4].copy_from_slice(colour);
// }

// fn initialise_viewport(window_width: u32, window_height: u32, window_scale: f64) -> ViewportData {
//     let event_loop = EventLoop::new();

//     let input = WinitInputHelper::new();

//     let window = {
//         let size = LogicalSize::new(window_width as f64, window_height as f64);
//         let scaled_size = LogicalSize::new(
//             window_scale * window_width as f64,
//             window_scale * window_height as f64,
//         );
//         WindowBuilder::new()
//             .with_title("Render Result")
//             .with_inner_size(size)
//             .with_min_inner_size(scaled_size)
//             .build(&event_loop)
//             .unwrap()
//     };

//     let pixels = {
//         let window_size = window.inner_size();
//         let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
//         Pixels::new(window_width, window_height, surface_texture)
//     }
//     .unwrap();

//     ViewportData {
//         pixels,
//         window,
//         event_loop,
//         input,
//     }
// }

// fn draw(
//     pixels: &mut Pixels,
//     colour_buffer: &Vec<ColourPosition>,
//     samples_per_pixel: i32,
//     window_height: u32,
//     window_width: u32,
// ) {
//     let frame = pixels.frame_mut();

//     for cp in colour_buffer {
//         let transformed_colour = drawn_colour(cp.colour, samples_per_pixel);
//         plot_pixel(
//             frame,
//             cp.point.0 as usize,
//             cp.point.1 as usize,
//             &transformed_colour,
//             window_width,
//             window_height,
//         );
//     }

//     pixels.render().unwrap();
// }
