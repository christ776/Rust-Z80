use std::time::Duration;

use Z80::{HEIGHT, WIDTH, memory::BoardMemory, memory::Memory, z80};
// use Z80::gfx_decoder::TileDecoder;

use crate::{CPU_CLOCK, Direction, Emulator, Machine, gfx_decoder::TileDecoder, utils::rom_loader::RomLoader};

impl Emulator for Machine {

    fn new() -> Self {
   
        Self {
            memory: BoardMemory::new(),
            dt: Duration::default(),
            pixel_buffer: vec![0; 65536],
            cpu: z80::Z80::new(),
            cycles_per_frame: CPU_CLOCK / 60,
            gfx_decoder: TileDecoder::new(WIDTH, HEIGHT),
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

    fn load_roms_numcrash(&mut self) {
        RomLoader::load_rom_mut(&String::from("./numcrash/nc-1.6e"), &mut self.memory.work_ram);
        RomLoader::load_rom_mut(&String::from("./numcrash/nc-5.6k"), &mut self.memory.work_ram);
        RomLoader::load_rom_mut(&String::from("./numcrash/nc-2.6f"), &mut self.memory.work_ram);
        RomLoader::load_rom_mut(&String::from("./numcrash/nc-6.6m"), &mut self.memory.work_ram);
        RomLoader::load_rom_mut(&String::from("./numcrash/nc-3.6h"), &mut self.memory.work_ram);
    }

    fn load_roms_pacman(&mut self) {
        // Code ROMs 
        RomLoader::load_rom_mut(&String::from("./pacman/pacman.6e"), &mut self.memory.work_ram);
        RomLoader::load_rom_mut(&String::from("./pacman/pacman.6f"), &mut self.memory.work_ram);
        RomLoader::load_rom_mut(&String::from("./pacman/pacman.6h"), &mut self.memory.work_ram);
        RomLoader::load_rom_mut(&String::from("./pacman/pacman.6j"), &mut self.memory.work_ram);
        //Tile ROM
        RomLoader::load_rom_mut(&String::from("./pacman/pacman.5e"), &mut self.memory.tile_rom);
        //Sprite ROM
        RomLoader::load_rom_mut(&String::from("./pacman/pacman.5f"), &mut self.memory.sprite_rom);

        //Color Rom
        let mut color_rom: Vec<u8> = vec![];
        RomLoader::load_rom_mut(&String::from("./pacman/82s123.7f"), &mut color_rom);
        let color_tables: Vec<u32> = color_rom.iter().map(|entry| {
            
            let mut red: u8 = 0; 
            if entry & 0x1 != 0 { red +=0x21; };
            if entry & 0x2 != 0 { red +=0x47; };
            if entry & 0x4 != 0 { red +=0x97; };

            let green = 
                if (entry & 0x8) != 0 { 0x21 } else { 0 } +
                if (entry & 0x10) != 0 { 0x47 } else { 0 } +
                if (entry & 0x20) != 0 { 0x97 } else { 0 };

            let blue = 
                if (entry & 0x40) != 0 { 0x51 } else { 0 } +
                if (entry & 0x80) != 0 { 0xAE } else { 0 };


            let result = [ red, green, blue, 0xff ];
            RomLoader::as_u32_be(&result)
        }).collect();
         
        //Palette Rom
        let mut palette_rom: Vec<u8> = vec![];
        RomLoader::load_rom_mut(&String::from("./pacman/82s126.4a"), &mut palette_rom);
        let mut palette_with_colors = vec![];
        let mut temp_storage: Vec<u32> = Vec::new();
        for (index,palette) in palette_rom.iter().enumerate() {
            if index % 4 == 0 && index != 0 {
                palette_with_colors.push(temp_storage.clone());
                temp_storage.clear();
            }
            if index == palette_rom.len() - 1 {
                temp_storage.push(color_tables[*palette as usize]);
                palette_with_colors.push(temp_storage.clone());
            }
    
            temp_storage.push(color_tables[*palette as usize]);
        }
        self.gfx_decoder.color_palettes = palette_with_colors;

        // Working RAM ... it's a bit of a hack for now
        // &mem.work_ram.append(&mut video_ram);
        let working_ram_size =  1024              // Video RAM (tile information)
                + 1024              // Video RAM (tile palettes)
                + 2032              // RAM
                + 16;               // Sprite number
        let mut working_ram:Vec<u8> = vec![0; working_ram_size];
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
    fn update(&mut self, dt: &Duration, direction: Direction, inserted_coin: bool, player1_start: bool) {
        let one_frame = Duration::new(0, 16_666_667);
        // Advance the timer by the delta time
        self.dt += *dt;
        // Trigger VBLANK interrupt? 
        let mut current_cycles = 0;
        while self.dt >= one_frame {
            while current_cycles <= self.cycles_per_frame {
                current_cycles += self.cpu.exec(&mut self.memory) as usize;
            }

            // Update Inputs
            if inserted_coin {
                println!("Inserted Coin!!")
            }

            let coin = if inserted_coin { 0x20 | self.memory.in0 } else { 0xDF & self.memory.in0 };
            let mut d: u8 = 0xff;
            match direction {
                Direction::Up => {
                    d = 0b1111_1110;
                }
                Direction::Down => {
                    d = 0b1111_0111;
                }
                Direction::Left => {
                    d = 0b1111_1101;
                }
                Direction::Right => {
                    d = 0b1111_1011;
                },
                _ => {}
            }

            self.memory.in0 = coin & d; 
            self.memory.in1 = if player1_start { 0xDF & self.memory.in1 } else { self.memory.in1 };

            // Update Gfx
            let sprite_rom = &self.memory.sprite_rom;
            let work_ram = &self.memory.work_ram;
            let tile_rom = &self.memory.tile_rom;
            self.gfx_decoder.decode_tile(&work_ram[0x4000..0x4400],
                &work_ram[0x4400..0x4800],
                &tile_rom, &mut self.pixel_buffer);
            self.gfx_decoder.decode_sprite(&self.memory.memory_mapped_area,
                 &work_ram[0x4ff0..=0x4FFF],
                 &&sprite_rom, &mut self.pixel_buffer);
            self.cpu.vblank();
            self.dt -= one_frame;
        }
    }
    
}