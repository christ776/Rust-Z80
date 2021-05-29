use crate::pixel::Pixel;

// Note that the columns 0, 1, 30 and 31 are not visible on the screen, 
// so the actual resolution is 28x36 characters, or 224x288 pixels.

pub struct TileDecoder {
  width: usize,
  height: usize,
  columns: usize,
  rows: usize,
  sprite_coordinates: Vec<u8>
}

impl TileDecoder {

  pub fn new(width: usize, height: usize) -> Self {
    Self {
      width,
      height,
      sprite_coordinates: vec![0; 16], // Sprite coordinates are stored in 16 bytes
      columns: width / 8,
      rows: height / 8 
        - 2 // Total rows is 36 minus 2 lower rows, renderer on their own
        - 2 // minus two top rows, rendered on thei own 
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
  /// spritetakes 16x16 pixels --> 256 pixels at 2bits each = 512bit --> 64bytes per sprite

  pub fn decode_tile(&self, video_ram: &[u8], tile_rom: &[u8], pixel_buffer: &mut Vec<u32>) {
    // Lower two rows
    let mut tile_offset = 0x02;
    for row in 34..=35 {
      // Columns need to be traversed backwards since the tile is stored from right to left
      for column in (2..30).rev() {
          let tile_data = video_ram[tile_offset] as usize; 
          match tile_rom.get(tile_data * 16..tile_data * 16 + 16) {
            Some(tile) => self.tile_to_pixels(tile, row, column, pixel_buffer),
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
    for row in 0..=1 {
      // Columns need to be traversed backwards since the tile is stored from right to left
      for column in (2..30).rev() { 
          let tile_data = video_ram[tile_offset] as usize; 
          match tile_rom.get(tile_data * 16..tile_data * 16 + 16) {
            Some(tile) => self.tile_to_pixels(tile, row, column, pixel_buffer),
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
      for row in 1..33 {
          let tile_offset = row + (27 - column) * self.rows + 0x40; 
          let tile_data = video_ram[tile_offset] as usize; 
          match tile_rom.get(tile_data * 16..tile_data * 16 + 16) {
            Some(tile) => self.tile_to_pixels(tile, row + 2, column, pixel_buffer),
            None => print!("Error?")
          }
      }
    }
  }

  pub fn update_sprite_coordinates(&mut self, address: usize, data: u8) {
    self.sprite_coordinates[address] = data
  }

  pub fn decode_sprite(&self, work_ram: &[u8], sprite_rom: &[u8], pixel_buffer: &mut Vec<u32>) {
    for sprite_number in 0..8 {
      let offset = sprite_number * 2;
      let columns  = 8;
      // The 4096 byte Sprite ROM (pacman.5f) stores 16x16 pixel sprites.
      // Each pixel uses 2 bits, resulting in each sprite using 64 bytes of ROM.
      // 4096/64 = 64 sprites stored in ROM
      let mut byte_offset: usize = (work_ram[offset] >> 2).into();
      let x_flip = (work_ram[offset] & 0x01) != 0;
      let y_flip = (work_ram[offset] & 0x02) != 0;
      byte_offset *= 64;
      // let byte_offset = 32 * 64 as usize;
      let offset_x: usize = self.sprite_coordinates[0 + offset] as usize;
      let offset_y = self.sprite_coordinates[1 + offset] as usize + 16 + 8;

      // The starting location for each 8 byte "group" is:
      // 5  1
      // 6  2
      // 7  3
      // 4  0

      let a = 8;
      let b = 16;
      self.draw_sprite_4_by_8(columns, sprite_rom, 
        byte_offset, 12, offset_y, offset_x + a,
            x_flip, y_flip, pixel_buffer); 

      self.draw_sprite_4_by_8(columns, sprite_rom, 
          byte_offset + 8, 0, offset_y, offset_x + a,
      x_flip, y_flip, pixel_buffer); 

      self.draw_sprite_4_by_8(columns, sprite_rom, 
        byte_offset + 16, 4, offset_y , offset_x + a,  
            x_flip, y_flip, pixel_buffer); 

      self.draw_sprite_4_by_8(columns, sprite_rom, 
        byte_offset + 24, 8, offset_y , offset_x + a, 
              x_flip, y_flip, pixel_buffer); 

      self.draw_sprite_4_by_8(columns, sprite_rom, 
        byte_offset + 32, 12, offset_y, offset_x,
            x_flip, y_flip, pixel_buffer); 

      self.draw_sprite_4_by_8(columns, sprite_rom, 
          byte_offset + 40, 0, offset_y, offset_x,
      x_flip, y_flip, pixel_buffer); 

      self.draw_sprite_4_by_8(columns, sprite_rom, 
        byte_offset + 48, 4, offset_y , offset_x, 
        x_flip, y_flip, pixel_buffer);  
      
      self.draw_sprite_4_by_8(columns, sprite_rom, 
        byte_offset + 56, 8, offset_y , offset_x,
        x_flip, y_flip, pixel_buffer); 
    }
  }

  fn draw_sprite_4_by_8(&self, columns: usize, 
      sprite_rom: &[u8],
      byte_offset: usize, vertical_offset: usize, 
      offset_y: usize, offset_x: usize,
      x_flip: bool, y_flip: bool,
      pixel_buffer: &mut Vec<u32>) 
    {

    let column_range = if x_flip { 8..0 } else { 0..8 };
    for column in column_range {
      // We need four bytes per 4 pixels , because each pixel in the pixel buffer has a 8-bit color depth
      // thus having 8 bit planes for 4 pixels
    
      //Get lowest four bits, each bit corresponding to a different pixel, plane 0
      let low_nibble = sprite_rom[(columns - 1 - column) + byte_offset] & 0x0F;
      //Get hightest four bits, each bit corresponding to a different pixel, plane 1
      let high_nibble = (sprite_rom[(columns - 1 - column) + byte_offset] >> 4) & 0x0F;
    
      let mut pixels =  [ 
        Pixel::new((low_nibble & 0x08) >> 3, (high_nibble & 0x08) >> 3),
        Pixel::new((low_nibble & 0x04) >> 2, (high_nibble & 0x04) >> 2),
        Pixel::new((low_nibble & 0x02) >> 1, (high_nibble & 0x02) >> 1),
        Pixel::new(low_nibble & 0x01, high_nibble & 0x01),
      ];
      if y_flip {
        pixels.reverse();
      }

      for (i, pixel) in pixels.iter().enumerate() {
        let raw_data = pixel.to_rgba();
        let pos = (i + vertical_offset)  
                * self.width // 224 pixles or 28 columns 
                + column  // column number
                + offset_y * self.width // row offset
                // * self.width * 8 
                + offset_x;
        pixel_buffer[pos] = raw_data;
      }
    }
  }

  fn tile_to_pixels(&self, tile: &[u8], offset_y: usize, offset_x: usize, pixel_buffer: &mut std::vec::Vec<u32>) {      
       // Upper Eight columns
      self.decode_columns(tile, offset_y, offset_x, pixel_buffer, 8, 0, 8);
        // Lower Eight columns
      self.decode_columns(tile, offset_y, offset_x, pixel_buffer, 8, 4, 0);
  }

fn decode_columns(&self, tile: &[u8], offset_y: usize, offset_x: usize,
     pixel_buffer: &mut Vec<u32>, columns: usize, vertical_offset: usize, byte_offset: usize) {
    for column in (0..columns).rev() {
        // We need four bytes per 4 pixels , because each pixel in the pixel buffer has a 8-bit color depth
        // thus having 8 bit planes for 4 pixels

        //Get lowest four bits, each bit corresponding to a different pixel, plane 0
        let low_nibble = tile[(columns - 1 - column) + byte_offset] & 0x0F;
        //Get hightest four bits, each bit corresponding to a different pixel, plane 1
        let high_nibble = (tile[(columns - 1 - column) + byte_offset] >> 4) & 0x0F;

        for (i, pixel) in [ 
          Pixel::new((low_nibble & 0x08) >> 3, (high_nibble & 0x08) >> 3),
          Pixel::new((low_nibble & 0x04) >> 2, (high_nibble & 0x04) >> 2),
          Pixel::new((low_nibble & 0x02) >> 1, (high_nibble & 0x02) >> 1),
          Pixel::new(low_nibble & 0x01, high_nibble & 0x01),
        ].iter().enumerate() {
            let raw_data = pixel.to_rgba();
            let pos = (i + vertical_offset)  
                    * self.width // 224 pixles or 28 columns 
                    + column  // column number
                    + offset_y // row offset
                    * self.width * 8 
                    + offset_x * 8;
            pixel_buffer[pos] = raw_data;
        }
      }
}
}