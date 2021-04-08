#![allow(non_snake_case)]
#[macro_use] extern crate log;
#[macro_use] extern crate bitflags;


/// In Z80 all 16-bit immedidates are encoded in the little-endian order of bytes,
/// meaning the byte that contains the least significant bits (LSB)
/// comes first and is followed by the byte that contains the most significant bits (MSB) of the value.

const HEIGHT: usize = 288;

pub mod z80;
pub const WIDTH: usize = 224;

mod memory;
mod gfx_decoder;
mod pixel;
mod gui;
mod registers;

use std::time::Duration;
use std::time::Instant;

use gui::Gui;
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::{LogicalPosition, LogicalSize, PhysicalSize};
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit_input_helper::WinitInputHelper;
use ::Z80::{memory::BoardMemory, memory::Memory, z80::Z80};


fn main () -> Result<(), Error> {
    let event_loop = EventLoop::new();
    let _input = WinitInputHelper::new();
    let (window, width, height, mut _hidpi_factor) = create_window("PacMan in Rust", &event_loop);
    let surface_texture = SurfaceTexture::new(width, height, &window);
    let mut pixels = Pixels::new(WIDTH as u32, HEIGHT as u32, surface_texture)?;
    let mut start_time = Instant::now();
    let mut last_frame = Instant::now();
    let mut world = Machine::new();
    world.load_roms();
    let mut input = WinitInputHelper::new();
    // Set up Dear ImGui
    let mut gui = Gui::new(&window, &pixels);
    let video_ram = world.memory.work_ram.get(0x4000..0x4400);
    match video_ram {
        Some(video_ram) => gui.set_memory_editor_mem(&video_ram),
        None => print!("Error?")
    }

    event_loop.run(move |event, _, control_flow| {
        // The one and only event that winit_input_helper doesn't have for us...
        if let Event::MainEventsCleared = event {
            window.request_redraw();
        }

        if let Event::RedrawEventsCleared = event {
            let now = Instant::now();
            gui.update_delta_time(now - last_frame);
            gui.update_cpu_state(&world.cpu);
            match world.memory.work_ram.get(0x4000..0x4400) {
                Some(video_ram) => gui.set_memory_editor_mem(&video_ram),
                None => print!("Error?")
            }

            last_frame = now;
        }

        if let Event::RedrawRequested(_) = event {
            world.draw(pixels.get_frame());

             // Prepare Dear ImGui
             gui.prepare(&window).expect("gui.prepare() failed");

              // Render everything together
            let render_result = pixels.render_with(|encoder, render_target, context| {
                // Render the world texture
                context.scaling_renderer.render(encoder, render_target);

                // Render Dear ImGui
                gui.render(&window, encoder, render_target, context)
                    .expect("gui.render() failed");
            });

            // Basic error handling
            if render_result
                .map_err(|e| error!("pixels.render() failed: {}", e))
                .is_err()
            {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        // Handle input events
        gui.handle_event(&window, &event);
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                pixels.resize(size.width, size.height);
            }

            // Update internal state and request a redraw
            let now = Instant::now();
            let dt = now.duration_since(start_time);
            start_time = now;
    
            // Update the game logic and request redraw
            world.update(&dt);
            window.request_redraw();
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

    });
}

struct Machine {
    cpu: Z80,
    memory: BoardMemory,
    dt: Duration,
    pixel_buffer: Vec<u32>,
}

impl Machine {    
     fn new() -> Self {

        Self {
            memory: BoardMemory::new(),
            dt: Duration::default(),
            cpu: Z80::new(),
            pixel_buffer: vec![0; 65536],
        }
    }

    fn load_roms(&mut self) {
        // Load ROM contents 
        Machine::load_rom_mut(&String::from("./pacman/pacman.6e"), &mut self.memory.work_ram);
        Machine::load_rom_mut(&String::from("./pacman/pacman.6f"), &mut self.memory.work_ram);
        Machine::load_rom_mut(&String::from("./pacman/pacman.6h"), &mut self.memory.work_ram);
        Machine::load_rom_mut(&String::from("./pacman/pacman.6j"), &mut self.memory.work_ram);
        //Tile ROM
        Machine::load_rom_mut(&String::from("./pacman/pacman.5e"), &mut self.memory.tile_rom);
        //Sprite ROM
        Machine::load_rom_mut(&String::from("./pacman/pacman.5f"), &mut self.memory.sprite_rom);

        // Working RAM ... it's a bit of a hack for now
        // &mem.work_ram.append(&mut video_ram);
        let mut working_ram:Vec<u8> = vec![0; 5220];
        self.memory.work_ram.append(&mut working_ram);
        // ; skip the checksum test, change 30fb to: ; HACK 0
        // ; 30fb  c37431    jp      #3174		; run the game!
        self.memory.work_ram[0x30fb as usize] = 0xc3;
        self.memory.work_ram[0x30fc as usize] = 0x74;
        self.memory.work_ram[0x30fd as usize] = 0x31;
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

        // Trigger VBLANK interrupt? 
        while self.dt >= one_frame {
            self.dt -= one_frame / 500;
            self.cpu.exec(&mut self.memory);
        }

        let sprite_rom = &self.memory.sprite_rom;
        let work_ram = &self.memory.work_ram;
        let tile_rom = &self.memory.tile_rom;
        self.memory.decoder.decode_tile(work_ram, &tile_rom, &mut self.pixel_buffer);
        self.memory.decoder.decode_sprite(&work_ram, &&sprite_rom, &mut self.pixel_buffer);
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
    /// Draw the `Machine` state to the frame buffer.
    ///
    /// Assumes the default texture format: [`wgpu::TextureFormat::Rgba8UnormSrgb`]
    fn draw(&mut self, frame: &mut [u8]) {
        // Clear the screen
            for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
                let t = self.pixel_buffer[i];
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
