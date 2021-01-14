#![allow(non_snake_case)]
#[macro_use] extern crate log;

/// In Z80 all 16-bit immedidates are encoded in the little-endian order of bytes,
/// meaning the byte that contains the least significant bits (LSB)
/// comes first and is followed by the byte that contains the most significant bits (MSB) of the value.

pub mod z80;
pub mod memory;
pub mod gfx_decoder;
pub mod pixel;

use std::time::Duration;
use std::time::Instant;
pub use crate::z80::*;

use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::{LogicalPosition, LogicalSize, PhysicalSize};
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit_input_helper::WinitInputHelper;

// /// The screen width is constant (units are in pixels)
// pub const SCREEN_WIDTH: usize = 224;
// /// The screen height is constant (units are in pixels)
// pub const SCREEN_HEIGHT: usize = 256;

const WIDTH: usize = 224;
const HEIGHT: usize = 288;

const TARGET_FPS: u64 = 60;


fn main () -> Result<(), Error> {
    let event_loop = EventLoop::new();
    let _input = WinitInputHelper::new();
    let (window, width, height, mut _hidpi_factor) = create_window("PacMan in Rust", &event_loop);
    let surface_texture = SurfaceTexture::new(width, height, &window);
    let mut pixels = Pixels::new(WIDTH as u32, HEIGHT as u32, surface_texture)?;
    let mut start_time = Instant::now();
    let mut world = World::new();
    let mut input = WinitInputHelper::new();

    world.emulator_init();

    // Set up Dear ImGui

    event_loop.run(move |event, _, control_flow| {
        // The one and only event that winit_input_helper doesn't have for us...
        if let Event::MainEventsCleared = event {
            // Application update code.

            // Queue a RedrawRequested event.
            //
            // You only need to call this if you've determined that you need to redraw, in
            // applications which do not always need to. Applications that redraw continuously
            // can just render here instead.
            let now = Instant::now();
            let dt = now.duration_since(start_time);
            start_time = now;
    
            // Update the game logic and request redraw
            world.update(&dt);
            window.request_redraw();
        }

        if let Event::RedrawRequested(_) = event {
            world.draw(pixels.get_frame());
            if pixels
                .render()
                .map_err(|e| error!("pixels.render() failed: {}", e))
                .is_err()
            {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        // Handle input events
       
        // let elapsed_time = Instant::now().duration_since(start_time).as_millis() as u64;

        // let wait_millis = match 1000 / TARGET_FPS >= elapsed_time {
        //     true => 1000 / TARGET_FPS - elapsed_time,
        //     false => 0
        // };
        // let new_inst = start_time + std::time::Duration::from_millis(wait_millis);
        // *control_flow = ControlFlow::WaitUntil(new_inst);
        


        // Get a new delta time.
        let now = Instant::now();
        let dt = now.duration_since(start_time);
        start_time = now;

        // Update the game logic and request redraw
        world.update(&dt);
        window.request_redraw();

        // For everything else, for let winit_input_helper collect events to build its state.
        // It returns `true` when it is time to update our game state and request a redraw.

    });
}

/// Representation of the application state. In this example, a box will bounce around the screen.
struct World {
    cpu: Z80,
    dt: Duration
}

impl World {
     /// Create a new `World` instance that can draw a moving box.
     fn new() -> Self {
        Self {
            cpu: z80::Z80::new(Memory::new_64k()),
            dt: Duration::default()
        }
    }

    fn emulator_init(&mut self) {
        //alloc memory
        let mut mem = Memory::new();
    
        // Load ROM contents 
        World::load_rom_mut(&String::from("./pacman/pacman.6e"), &mut mem.work_ram);
        World::load_rom_mut(&String::from("./pacman/pacman.6f"), &mut mem.work_ram);
        World::load_rom_mut(&String::from("./pacman/pacman.6h"), &mut mem.work_ram);
        World::load_rom_mut(&String::from("./pacman/pacman.6j"), &mut mem.work_ram);
        //Tile ROMS
        World::load_rom_mut(&String::from("./pacman/pacman.5e"), &mut mem.tile_rom);
        World::load_rom_mut(&String::from("./pacman/pacman.5f"), &mut mem.tile_rom);


        // Working RAM ... it's a bit of a hack for now
        let mut video_ram:Vec<u8> = vec![0; 2048];
        &mem.work_ram.append(&mut video_ram);
        let mut working_ram:Vec<u8> = vec![0; 4196];
        &mem.work_ram.append(&mut working_ram);
        println!("Memory size is {}", format!("{:#x}", mem.work_ram.len()));

        mem.pixel_buffer = vec![0; 64512];

        self.cpu = z80::Z80::new(mem);
    }


    /// Update the internal state.
    ///
    /// # Arguments
    ///
    /// * `dt`: The time delta since last update.
    /// * `controls`: The player inputs.
    pub fn update(&mut self, dt: &Duration) {
        let one_frame = Duration::new(0, 16_666_667);
        // Advance the timer by the delta time
        self.dt += *dt;

        //Trigger VBLANK interrupt? 
        while self.dt >= one_frame {
            self.dt -= one_frame / 500;
            self.cpu.exec();
        }

        self.cpu.vblank();
    }
    
    fn load_rom_mut(rom_name: &String, mem: &mut Vec<u8>) {
        match std::fs::read(rom_name) {
            Ok(bytes) => { 
                let mut buffer: Vec<u8> = bytes;            
                mem.append(&mut buffer);
            }
            Err(e) => {
                if e.kind() == std::io::ErrorKind::PermissionDenied {
                    eprintln!("please run again with appropriate permissions.");
                    return;
                }
                panic!("{}", e);
            }
        }
    }
    /// Draw the `World` state to the frame buffer.
    ///
    /// Assumes the default texture format: [`wgpu::TextureFormat::Rgba8UnormSrgb`]
    fn draw(&mut self, frame: &mut [u8]) {
        // Clear the screen
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {

            let x = i % WIDTH as usize;
            let y = i / WIDTH as usize;

            let t = self.cpu.mem.pixel_buffer[x + y * WIDTH];
            let raw_bytes = t.to_be_bytes();
            pixel.copy_from_slice(&raw_bytes);
        }
    }
}

/// Create a window for the game.
///
/// Automatically scales the window to cover about 2/3 of the monitor height.
///
/// # Returns
///
/// Tuple of `(window, surface, width, height, hidpi_factor)`
/// `width` and `height` are in `PhysicalSize` units.
fn create_window(title: &str, event_loop: &EventLoop<()>,
) -> (winit::window::Window, u32, u32, f64) {
    // Create a hidden window so we can estimate a good default window size
    let window = winit::window::WindowBuilder::new()
        .with_visible(false)
        .with_title(title)
        .build(&event_loop)
        .unwrap();
    let hidpi_factor = window.scale_factor();

    // Get dimensions
    let width = WIDTH as f64;
    let height = HEIGHT as f64;
    let (monitor_width, monitor_height) = {
        let size = window.current_monitor().unwrap().size();
        (
            size.width as f64 / hidpi_factor,
            size.height as f64 / hidpi_factor,
        )
    };
    let scale = (monitor_height / height * 2.0 / 3.0).round();

    // Resize, center, and display the window
    let min_size = PhysicalSize::new(width, height).to_logical::<f64>(hidpi_factor);
    let default_size = LogicalSize::new(width * scale, height * scale);
    let center = LogicalPosition::new(
        (monitor_width - width * scale) / 2.0,
        (monitor_height - height * scale) / 2.0,
    );
    window.set_inner_size(default_size);
    window.set_min_inner_size(Some(min_size));
    window.set_outer_position(center);
    window.set_visible(true);

    let size = default_size.to_physical::<f64>(hidpi_factor);

    (
        window,
        size.width.round() as u32,
        size.height.round() as u32,
        hidpi_factor,
    )
}
