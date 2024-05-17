use crate::nes::cpu::Cpu;
use crate::nes::rom::Rom;

use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event::{Event, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;


pub struct Emulator {
    cpu: Cpu
}

impl Emulator {
    pub fn new(rom: Rom) -> Emulator {
        Emulator {
            cpu: Cpu::new(rom)
        }
    }

    pub fn run(mut self) {


        self.cpu.reset();
        // self.cpu.run();

        let event_loop = EventLoop::new();
        let mut input = WinitInputHelper::new();

        const WIDTH: u32 = 256 + 100;
        const HEIGHT: u32 = 240;

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

        let mut pixels = {
            let window_size = window.inner_size();
            let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
            Pixels::new(WIDTH, HEIGHT, surface_texture).expect("Cant pixel yo")
        };

        event_loop.run(move |event, _, control_flow| {
            if let Event::RedrawRequested(_) = event {
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
                // Resize the window
                if let Some(size) = input.window_resized() {
                    if let Err(err) = pixels.resize_surface(size.width, size.height) {
                        eprintln!("Error: pixels.resize_surface: {}", err);
                        *control_flow = ControlFlow::Exit;
                        return;
                    }
                }


                window.request_redraw();
            }
        });


    }
}
