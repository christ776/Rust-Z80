use std::{fs::File, io::BufWriter, path::Path};
use Z80::utils::rom_loader::RomLoader;

#[cfg(test)]
// #[test]
fn test_palette() {
    let mut color_rom: Vec<u8> = vec![];

    // Color Rom
    RomLoader::load_rom_mut(&String::from(r"./pacman/82s123.7f"), &mut color_rom);
    let color_tables: Vec<[u8; 4]>  = color_rom.iter().map(|entry| {
            
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

        [ red, green, blue, 0xff ]
    }).collect();

    //Palette Rom
    let mut palette_rom: Vec<u8> = vec![];
    RomLoader::load_rom_mut(&String::from(r"./pacman/82s126.4a"), &mut palette_rom);

    let mut palette_with_colors = vec![];
    let mut temp_storage: Vec<u8> = Vec::new();
    for (index,palette) in palette_rom.iter().enumerate() {
        if index % 4 == 0 && index != 0 {
            palette_with_colors.push(temp_storage.clone());
            temp_storage.clear();
        }
        if index == palette_rom.len() - 1 {
            temp_storage.push(*palette);
            palette_with_colors.push(temp_storage.clone());
        }

        temp_storage.push(*palette);
    }

    let path = Path::new(r"./image.png");
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);
    
    let mut encoder = png::Encoder::new(w, 64, 4); // Width is 2 pixels and height is 1.
    encoder.set_color(png::ColorType::RGBA);
    encoder.set_depth(png::BitDepth::Eight);

    let mut data = vec![]; // 512 u8 numbers

    for row in 0..=3 {
        for column in 0..=63 {
            let color_index = palette_with_colors[column][row];
            let colors = color_tables[color_index as usize];
            data.push(colors[0]);
            data.push(colors[1]);
            data.push(colors[2]);
            data.push(colors[3]);
        }
    }

    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(&data).unwrap();
}


