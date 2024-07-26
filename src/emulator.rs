use std::sync::{Arc, mpsc, Mutex};
use std::thread;
use crate::gui::Framework;
use crate::nes::cpu::{Cpu, HEIGHT, RenderImage, WIDTH};
use crate::nes::rom::Cartridge;

use pixels::{Pixels, SurfaceTexture};
use rand::random;
use winit::{
    dpi::LogicalSize,
    event::{Event, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;



pub struct Emulator {
    cpu: Cpu,
    image: Arc<Mutex<RenderImage>>,
}

impl Emulator {
    pub fn new(cartridge: Cartridge) -> Emulator {
        let image: Arc<Mutex<RenderImage>> = Arc::new(Mutex::new([0; (WIDTH * HEIGHT * 4) as usize].to_vec()));

        Emulator {
            cpu: Cpu::new(cartridge, image.clone()),
            image
        }
    }

    pub fn run(mut self) {



        self.cpu.reset();

        let event_loop = EventLoop::new();
        let mut input = WinitInputHelper::new();

        let window = {
            let size = LogicalSize::new(WIDTH, HEIGHT);
            let scaled_size = LogicalSize::new(WIDTH as f64 * 3.0, HEIGHT as f64 * 3.0);

            WindowBuilder::new()
                .with_title("BunNES")
                .with_inner_size(scaled_size)
                .with_min_inner_size(size)
                .build(&event_loop)
                .unwrap()
        };

        let (mut pixels, mut framework) = {
            let window_size = window.inner_size();
            let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
            let scale_factor = window.scale_factor() as f32;
            let pixels = Pixels::new(WIDTH, HEIGHT, surface_texture).expect("Could not create pixels");

            let framework = Framework::new(
                &event_loop,
                window_size.width,
                window_size.height,
                scale_factor,
                &pixels,
            );

            (pixels, framework)
        };

        thread::spawn(move || {
            self.cpu.run();
        });



        event_loop.run(move |event, _, control_flow| {


            if let Event::RedrawRequested(_) = event {

                let frame = pixels.frame_mut();


                // println!("emulator render to screen");
                let image = self.image.lock().unwrap();
                frame.copy_from_slice(&image);


                if let Err(err) = pixels.render() {
                    eprintln!("Error: pixels.render: {}", err);
                    *control_flow = ControlFlow::Exit;
                    return;
                }
            }

            // For everything else, for let winit_input_helper collect events to build its state.
            // It returns `true` when it is time to update our game state and request a redraw.
            if input.update(&event) {
                // Close events
                if input.key_pressed(VirtualKeyCode::Escape) || input.close_requested() {
                    *control_flow = ControlFlow::Exit;
                    return;
                }

                if let Some(scale_factor) = input.scale_factor() {
                    framework.scale_factor(scale_factor);
                }

                // Resize the window
                if let Some(size) = input.window_resized() {
                    if let Err(err) = pixels.resize_surface(size.width, size.height) {
                        eprintln!("Error: pixels.resize_surface: {}", err);
                        *control_flow = ControlFlow::Exit;
                        return;
                    }

                    framework.resize(size.width, size.height);
                }


                window.request_redraw();
            }

            match event {
                Event::WindowEvent {event, ..} => {
                    framework.handle_event(&event);
                }

                Event::RedrawRequested(_) => {
                    // Prepare egui
                    framework.prepare(&window);

                    // Render everything together
                    let render_result = pixels.render_with(|encoder, render_target, context| {
                        // Render the world texture
                        context.scaling_renderer.render(encoder, render_target);

                        // Render egui
                        framework.render(encoder, render_target, context);

                        Ok(())
                    });

                    // Basic error handling
                    if let Err(err) = render_result {
                        eprintln!("Error: pixels.render: {}", err);
                        *control_flow = ControlFlow::Exit;
                    }
                }

                _ => {}
            }
        });


    }
}
