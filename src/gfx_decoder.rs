use crate::pixel::Pixel;

pub struct TileDecoder {
  width: usize
}

impl TileDecoder {

  pub fn new(width: usize) -> Self {
    Self {
      width
    }
  }

  /// Since screen is made up of 224 x 288 pixels (rotated)
  /// and each tile is a 8x8 pixel 4-color square, we could then split the ROM contents
  /// into a 28 x 36 tileset.
  /// Also, given that each tile is a 8x8 pixel and each pixel has 2-bit color depth, then each
  /// tile takes 8x8 pixels --> 64 pixels at 2bits each = 128bit --> 16bytes per tile
  /// |--------
  /// | 4 | 4 |
  /// |--------
  /// | 4 | 4 |
  /// |--------
  /// Sprites on the other hand are stored as 16x16 pixels
  /// |--------
  /// | 8 | 8 |
  /// |--------
  /// | 8 | 8 |
  /// |--------
  /// Likewise Sprites have 16x16 pixels and each pixel has 2-bit color depth, then each
  /// spritetakes 16x16 pixels --> 256 pixels at 2bits each = 512bit --> 64bytes per tile

  pub fn decode_tile(&self, video_ram: &[u8], tile_rom: &[u8], pixel_buffer: &mut Vec<u32>) {

    // match tile_rom.get(tile_data * tile..tile_data * tile + 16) {
    //   Some(tile) => self.to_pixel_buffer(tile, row, column, pixel_buffer),
    //   None => print!("Error?")
    // }

    // for tile in video_ram.iter() {
    //   if *tile >= 0x02 && *tile <= 0x1d || *tile >= 0x22 && *tile <= 0x03d { 
  
    //     if tile_offset == 0x1d {
    //       tile_offset = 0x22;
    //     } else {
    //       tile_offset += 1; 
    //     }
    // }

    // Lower two rows
    let mut tile_offset = 0x02;
    for row in 34..=35 {
      // Columns need to be traversed backwards since the tile is stored from right to left
      for column in (2..30).rev() {
          let tile_data = video_ram[tile_offset + 0x4000] as usize; 
          match tile_rom.get(tile_data * 16..tile_data * 16 + 16) {
            Some(tile) => self.to_pixel_buffer(tile, row, column, pixel_buffer),
            None => print!("Error?")
          }
          if tile_offset == 0x1d {
            tile_offset = 0x22;
          } else {
            tile_offset += 1; 
          }
      }
    }

    // Top two rows
    tile_offset = 0x03c2;
    for row in 0..3 {
      // Columns need to be traversed backwards since the tile is stored from right to left
      for column in (2..30).rev() { 
          let tile_data = video_ram[tile_offset + 0x4000] as usize; 
          match tile_rom.get(tile_data * 16..tile_data * 16 + 16) {
            Some(tile) => self.to_pixel_buffer(tile, row, column, pixel_buffer),
            None => print!("Error?")
          }
          if tile_offset == 0x03dd {
            tile_offset = 0x03e2;
          } else {
            tile_offset += 1; 
          }
      }
    }

    //Middle rows
    for column in 0..28 {
      for row in 0..32 {
          let tile_offset = row + (27 - column) * 32 + 0x40; 
          let tile_data = video_ram[tile_offset + 0x4000] as usize; 
          match tile_rom.get(tile_data * 16..tile_data * 16 + 16) {
            Some(tile) => self.to_pixel_buffer(tile, row, column, pixel_buffer),
            None => print!("Error?")
          }
      }
      // offset += 4;
    }
  }

  pub fn decode_sprite(&self, work_ram: &[u8], sprite_rom: &[u8], pixel_buffer: &mut Vec<u32>) {
    for sprite_number in 0..8 {
      let offset = 0x5060 + sprite_number * 2;
      let x_coord = work_ram[offset] as usize;
      let y_coord = work_ram[offset + 1] as usize;
            
       // Lower Sixteen columns
       for column in 0..16 {

        //Get lowest four bits, each bit corresponding to a different pixel, plane 0
        let low_nibble = sprite_rom[column] & 0x0F;
        //Get hightest four bits, each bit corresponding to a different pixel, plane 1
        let high_nibble = (sprite_rom[column] >> 4) & 0x0F;

        for (i, pixel) in [ 
          Pixel::new(low_nibble & 0x01, high_nibble & 0x01),
          Pixel::new((low_nibble & 0x02) >> 1, (high_nibble & 0x02) >> 1),
          Pixel::new((low_nibble & 0x04) >> 2, (high_nibble & 0x04) >> 2),
          Pixel::new((low_nibble & 0x08) >> 3, (high_nibble & 0x08) >> 3) 
        ].iter().enumerate() {
            let raw_data = pixel.to_rgba();
            let pos = (i + 4) * self.width + column + y_coord * self.width * 8 + x_coord * 8;
            pixel_buffer[pos] = raw_data;
        }
      }
    }
  }

 fn to_pixel_buffer(&self, tile: &[u8], offset_y: usize, offset_x: usize, pixel_buffer: &mut std::vec::Vec<u32>) {      
       // Upper Eight columns
      for column in (0..8).rev() {
        // We need four bytes per 4 pixels , because each pixel in the pixel buffer has a 8-bit color depth
        // thus having 8 bit planes for 4 pixels

        //Get lowest four bits, each bit corresponding to a different pixel, plane 0
        let low_nibble = tile[ (7 - column) + 8] & 0x0F;
        //Get hightest four bits, each bit corresponding to a different pixel, plane 1
        let high_nibble = (tile[(7 - column) + 8] >> 4) & 0x0F;

        for (i, pixel) in [ 
          Pixel::new((low_nibble & 0x08) >> 3, (high_nibble & 0x08) >> 3),
          Pixel::new((low_nibble & 0x04) >> 2, (high_nibble & 0x04) >> 2),
          Pixel::new((low_nibble & 0x02) >> 1, (high_nibble & 0x02) >> 1),
          Pixel::new(low_nibble & 0x01, high_nibble & 0x01),
        ].iter().enumerate() {
            let raw_data = pixel.to_rgba();
            let pos = i 
                    * self.width // 224 pixles or 28 columns 
                    + column  // column number
                    + offset_y // row offset
                    * self.width * 8 
                    + offset_x * 8;
            pixel_buffer[pos] = raw_data;
        }
      }

       // Lower Eight columns
      for column in (0..8).rev() {
        //We need four bytes per 4 pixels , because each pixel has a 8-bit color depth
        // thus having 8 bit planes for 4 pixels

        //Get lowest four bits, each bit corresponding to a different pixel, plane 0
        let low_nibble = tile[7 - column] & 0x0F;
        //Get hightest four bits, each bit corresponding to a different pixel, plane 1
        let high_nibble = (tile[7 - column] >> 4) & 0x0F;

        for (i, pixel) in [ 
          Pixel::new((low_nibble & 0x08) >> 3, (high_nibble & 0x08) >> 3),
          Pixel::new((low_nibble & 0x04) >> 2, (high_nibble & 0x04) >> 2),
          Pixel::new((low_nibble & 0x02) >> 1, (high_nibble & 0x02) >> 1),
          Pixel::new(low_nibble & 0x01, high_nibble & 0x01),
        ].iter().enumerate() {
            let raw_data = pixel.to_rgba();
            let pos = (i + 4)
                    * self.width
                    + column
                    + offset_y
                    * self.width * 8
                    + offset_x * 8;
            pixel_buffer[pos] = raw_data;
        }
    }
  }
}