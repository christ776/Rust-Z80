#![allow(non_snake_case)]
#[macro_use] extern crate log;
#[macro_use] extern crate bitflags;


/// In Z80 all 16-bit immedidates are encoded in the little-endian order of bytes,
/// meaning the byte that contains the least significant bits (LSB)
/// comes first and is followed by the byte that contains the most significant bits (MSB) of the value.

const HEIGHT: usize = 288;
const CPU_CLOCK: usize = 3072000;

pub mod z80;
pub const WIDTH: usize = 224;

mod memory;
mod gfx_decoder;
mod gui;
mod registers;
mod pacman;
mod utils;

use std::env;
use std::time::Duration;
use std::time::Instant;

use ::Z80::memory::Memory;
use gfx_decoder::TileDecoder;
use gilrs::{Button, Gilrs};
use gui::Gui;
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::{LogicalPosition, LogicalSize, PhysicalSize};
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit_input_helper::WinitInputHelper;
use ::Z80::{memory::BoardMemory, z80::Z80};

pub enum Direction {
    Up,
    Down,
    Right,
    Left,
    Still
}


fn main () -> Result<(), Error> {

    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        println!("Game to load: {}", args[1]);
        let mut emulator = Machine::new();
        let game_name = &args[1];
        match game_name.as_str() {
            "pacman" => {
                emulator.load_roms_pacman();
            },
            "numcrash" => emulator.load_roms_numcrash(),
            _ => {}
        }
    }

    let event_loop = EventLoop::new();
    let _input = WinitInputHelper::new();
    let (window, width, height, mut _hidpi_factor) = create_window("PacMan in Rust", &event_loop);
    let surface_texture = SurfaceTexture::new(width, height, &window);
    let mut pixels = Pixels::new(WIDTH as u32, HEIGHT as u32, surface_texture)?;
    let mut start_time = Instant::now();
    let mut last_frame = Instant::now();
    let mut emulator = Machine::new();
    emulator.load_roms_pacman();
    let mut input = WinitInputHelper::new();

    // Gamepads

    let mut gilrs = Gilrs::new().unwrap();
    // Iterate over all connected gamepads
    for (_id, gamepad) in gilrs.gamepads() {
        println!("{} is {:?}", gamepad.name(), gamepad.power_info());
    }
    let mut gamepad = None;

    // Set up Dear ImGui
    let mut gui = Gui::new(&window, &pixels);
    // let video_ram = emulator.memory.work_ram.get(0x4000..0x4400);
    // match video_ram {
    //     Some(video_ram) => gui.set_memory_editor_mem(&video_ram),
    //     None => print!("Error?")
    // }

    event_loop.run(move |event, _, control_flow| {
        // The one and only event that winit_input_helper doesn't have for us...

        if let Event::RedrawEventsCleared = event {
            let now = Instant::now();
            gui.update_delta_time(now - last_frame);
            gui.update_cpu_state(&emulator.cpu);
            gui.update_in0(&emulator.memory.r8(0x5000));

            last_frame = now;
        }

        // Pump the gilrs event loop and find an active gamepad
        // Examine new events
        while let Some(gilrs::Event { id, event: _, time: _ }) = gilrs.next_event() {
            // println!("{:?} New event from {}: {:?}", time, id, event);
            gamepad = Some(id);
        }

        // You can also use cached gamepad state
        if let Some(gamepad) = gamepad.map(|id| gilrs.gamepad(id)) {
            if gamepad.is_pressed(Button::South) {
                println!("Button South is pressed (XBox - A, PS - X)");
            }
        }

        if let Event::RedrawRequested(_) = event {
            emulator.draw(pixels.get_frame());

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

            // Keyboard controls
            let mut left = input.key_held(VirtualKeyCode::Left);
            let mut right = input.key_held(VirtualKeyCode::Right);
            let mut up = input.key_held(VirtualKeyCode::Up);
            let mut down = input.key_held(VirtualKeyCode::Down);
            // let mut fire = input.key_pressed(VirtualKeyCode::Space);

            let insert_coin = input.key_held(VirtualKeyCode::Key5);
            let player1_start = input.key_held(VirtualKeyCode::Key1);

            // Gamepad controls
            if let Some(id) = gamepad {
                let gamepad = gilrs.gamepad(id);

                left = left || gamepad.is_pressed(Button::DPadLeft);
                right = right || gamepad.is_pressed(Button::DPadRight);
                up = up || gamepad.is_pressed(Button::DPadUp);
                down = down || gamepad.is_pressed(Button::DPadDown);
                // fire = fire
                //     || gamepad.button_data(Button::South).map_or(false, |button| {
                //         button.is_pressed() && button.counter() == gilrs.counter()
                //     });
            }

            let mut direction = Direction::Still;
            if left {
                direction = Direction::Left
            } 
            if right {
                direction = Direction::Right
            } 
            if up {
                direction = Direction::Up
            } 
            if down {
                direction = Direction::Down
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                pixels.resize_surface(size.width, size.height);
            }

            // Update internal state and request a redraw
            let now = Instant::now();
            let dt = now.duration_since(start_time);
            start_time = now;

            // Update the game logic and request redraw
            emulator.update(&dt, direction, insert_coin, player1_start);
            window.request_redraw();
        }
    });
}

pub trait Emulator {
    fn new() -> Self;
    fn load_roms_pacman(&mut self);
    fn load_roms_numcrash(&mut self);
    fn draw(&mut self, frame: &mut [u8]);
    fn update(&mut self, 
        dt: &Duration, 
        direction: Direction, inserted_coin: bool, 
        player1_start: bool);
}

struct Machine {
    cpu: Z80,
    memory: BoardMemory,
    dt: Duration,
    pixel_buffer: Vec<u32>,
    cycles_per_frame: usize,
    gfx_decoder:TileDecoder,
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
