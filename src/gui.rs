use imgui_memory_editor::MemoryEditor;
use std::time::Duration;
use imgui::*;
use pixels::{wgpu, PixelsContext};
use std::time::Instant;

/// Manages all state required for rendering Dear ImGui over `Pixels`.
pub(crate) struct Gui {
    imgui: imgui::Context,
    platform: imgui_winit_support::WinitPlatform,
    renderer: imgui_wgpu::Renderer,
    last_frame: Instant,
    last_cursor: Option<imgui::MouseCursor>,
    about_open: bool,
    delta_s: Duration,
    pc: u16,
    // video_memory_editor: &'a Vec<u8>
}

impl Gui {
    /// Create Dear ImGui.
    pub(crate) fn new(window: &winit::window::Window, pixels: &pixels::Pixels) -> Self {
        // Create Dear ImGui context
        let mut imgui = imgui::Context::create();
        imgui.set_ini_filename(None);

        // Initialize winit platform support
        let mut platform = imgui_winit_support::WinitPlatform::init(&mut imgui);
        platform.attach_window(
            imgui.io_mut(),
            window,
            imgui_winit_support::HiDpiMode::Default,
        );

        // Configure Dear ImGui fonts
        let hidpi_factor = window.scale_factor();
        let font_size = (13.0 * hidpi_factor) as f32;
        imgui.io_mut().font_global_scale = (1.0 / hidpi_factor) as f32;
        imgui
            .fonts()
            .add_font(&[imgui::FontSource::DefaultFontData {
                config: Some(imgui::FontConfig {
                    oversample_h: 1,
                    pixel_snap_h: true,
                    size_pixels: font_size,
                    ..Default::default()
                }),
            }]);

        // Fix incorrect colors with sRGB framebuffer
        let style = imgui.style_mut();
        for color in 0..style.colors.len() {
            style.colors[color] = gamma_to_linear(style.colors[color]);
        }

        // Create Dear ImGui WGPU renderer
        let device = pixels.device();
        let queue = pixels.queue();
        let texture_format = wgpu::TextureFormat::Bgra8UnormSrgb;
        let config = imgui_wgpu::RendererConfig {
            texture_format,
            ..Default::default()
        };
        let renderer = imgui_wgpu::Renderer::new(&mut imgui, &device, &queue, config);

        // Return GUI context
        Self {
            imgui,
            platform,
            renderer,
            last_frame: Instant::now(),
            last_cursor: None,
            about_open: true,
            delta_s: Duration::new(0, 0),
            pc: 0,
            // video_memory_editor: vec![0; 1024]
        }
    }

    /// Prepare Dear ImGui.
    pub(crate) fn prepare(
        &mut self,
        window: &winit::window::Window,
    ) -> Result<(), winit::error::ExternalError> {
        // Prepare Dear ImGui
        let now = Instant::now();
        self.imgui.io_mut().update_delta_time(now - self.last_frame);
        self.last_frame = now;
        self.platform.prepare_frame(self.imgui.io_mut(), window)
    }

    /// Render Dear ImGui.
    pub(crate) fn render(
        &mut self,
        window: &winit::window::Window,
        encoder: &mut wgpu::CommandEncoder,
        render_target: &wgpu::TextureView,
        context: &PixelsContext,
    ) -> imgui_wgpu::RendererResult<()> {
        // Start a new Dear ImGui frame and update the cursor
        let ui = self.imgui.frame();

        let mouse_cursor = ui.mouse_cursor();
        if self.last_cursor != mouse_cursor {
            self.last_cursor = mouse_cursor;
            self.platform.prepare_render(&ui, window);
        }

        // Draw windows and GUI elements here
        let mut about_open = false;
        ui.main_menu_bar(|| {
            ui.menu(imgui::im_str!("Help"), true, || {
                about_open = imgui::MenuItem::new(imgui::im_str!("About...")).build(&ui);
            });
        });
        if about_open {
            self.about_open = true;
        }

        if self.about_open {
            ui.show_about_window(&mut self.about_open);
        }

        // let window = imgui::Window::new(im_str!("Hello world"));
        // window
        //     .size([300.0, 100.0], Condition::FirstUseEver)
        //     .build(&ui, || {
        //         ui.text(im_str!("Hello world!"));
        //         ui.text(im_str!("This...is...imgui-rs on WGPU!"));
        //         ui.separator();
        //         let mouse_pos = ui.io().mouse_pos;
        //         ui.text(im_str!(
        //             "Mouse Position: ({:.1},{:.1})",
        //             mouse_pos[0],
        //             mouse_pos[1]
        //         ));
        //     });

        // Let's try a Memory Editor!
        let vec = vec![0; 0x400];
        // Can also use a &mut [u8] if you want to use the editor to modify the slice
        let mut memory_editor = MemoryEditor::<&[u8]>::new()
        .draw_window(im_str!("Memory")) // Can omit if you don't want to create a window
        .read_only(false);

        if memory_editor.open() { // open() can be omitted if draw_window was not used
            memory_editor.draw_vec(&ui, &vec)
        }

        let window = imgui::Window::new(im_str!("CPU and FPS"));
        let delta = self.delta_s;
        let pc = self.pc;

        window
            .size([400.0, 200.0], Condition::FirstUseEver)
            .position([400.0, 200.0], Condition::FirstUseEver)
            .build(&ui, || {
                ui.text(im_str!("Frametime: {:?}",  delta));
                ui.separator();
                ui.text(im_str!("PC: {:?}",format!("{:#x}", pc)));
            });

        // Render Dear ImGui with WGPU
        let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                attachment: render_target,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: true,
                },
            }],
            depth_stencil_attachment: None,
        });

        self.renderer
            .render(ui.render(), &context.queue, &context.device, &mut rpass)
    }

    /// Handle any outstanding events.
    pub(crate) fn handle_event(
        &mut self,
        window: &winit::window::Window,
        event: &winit::event::Event<()>,
    ) {
        self.platform
            .handle_event(self.imgui.io_mut(), window, event);
    }

    pub fn update_delta_time(&mut self, delta: std::time::Duration) {
        self.delta_s = delta
    }

    pub fn update_cpu_state(&mut self, pc: u16) {
        self.pc = pc
    }

    // pub fn set_memory_editor_mem(&mut self, data: &Vec<u8>) {
    //     self.video_memory_editor = data
    // }
}

fn gamma_to_linear(color: [f32; 4]) -> [f32; 4] {
    const GAMMA: f32 = 2.2;

    let x = color[0].powf(GAMMA);
    let y = color[1].powf(GAMMA);
    let z = color[2].powf(GAMMA);
    let w = 1.0 - (1.0 - color[3]).powf(GAMMA);

    [x, y, z, w]
}
