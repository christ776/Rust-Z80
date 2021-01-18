
use crate::pixel::Pixel;

pub struct TileDecoder {
  width: usize
}


pub trait Decoder {
  fn decode_tile(&self, offset: usize, tile_rom: &Vec<u8>, tile: usize, pixel_buffer: &mut Vec<u32>);
  fn to_pixel_buffer(&self, offset: usize, tile: &[u8], pixel_buffer: &mut std::vec::Vec<u32>);
  fn decode_sprite(&self, offset: usize, sprite_rom: &Vec<u8>, pixel_buffer: &mut Vec<u32>); 
  fn new(width: usize) -> Self where Self: Sized;
}

impl Decoder for TileDecoder {

  fn new(width: usize) -> Self {
    Self {
      width: width
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
  fn decode_tile(&self, offset: usize, tile_rom: &Vec<u8>, tile: usize, pixel_buffer: &mut Vec<u32>) {
    let tile_offset = tile * 16;
    match &tile_rom.get(tile_offset ..tile_offset + 16) {
      Some(tile) => self.to_pixel_buffer(offset, tile, pixel_buffer),
      None => print!("Error?")
    }
  }

  fn decode_sprite(&self, offset: usize, sprite_rom: &Vec<u8>, pixel_buffer: &mut Vec<u32>) {
      match &sprite_rom.get(offset ..offset + 64) {
        Some(tile) => self.to_pixel_buffer(offset, tile, pixel_buffer),
        None => print!("Error?")
      }
  }

  fn to_pixel_buffer(&self, offset: usize, tile: &[u8], pixel_buffer: &mut std::vec::Vec<u32>) {
    if offset < 0x40 {
      return
    }
      let offset_y = (offset - 0x40) % 36 as usize;
      let offset_x = (offset - 0x40) / 36 as usize;
      
       // Upper Eight columns
      for column in 0..8 {
        //We need four bytes per 4 pixels , because each pixel has a 8-bit color depth
        // thus having 8 bit planes for 4 pixels

        //Get lowest four bits, each bit corresponding to a different pixel, plane 0
        let low_nibble = tile[column + 8] & 0x0F;
        //Get hightest four bits, each bit corresponding to a different pixel, plane 1
        let high_nibble = (tile[column + 8] >> 4) & 0x0F;

        for (i, pixel) in [ 
          Pixel::new(low_nibble & 0x01, high_nibble & 0x01),
          Pixel::new((low_nibble & 0x02) >> 1, (high_nibble & 0x02) >> 1),
          Pixel::new((low_nibble & 0x04) >> 2, (high_nibble & 0x04) >> 2),
          Pixel::new((low_nibble & 0x08) >> 3, (high_nibble & 0x08) >> 3) 
        ].iter().enumerate() {
            let raw_data = pixel.to_rgba();
            let pos = (i + 4) * self.width + column + offset_y * self.width * 8 + offset_x * 8;
            // if column == 7 && i == 3 {
            //   println!("Last position for upper eight: {}", format!("{:#x}", pos));
            // }
            // if column == 0 && i == 0 {
            //   println!("First position for upper eight: {}", format!("{:#x}", pos));
            // }
            pixel_buffer[pos] = raw_data;
        }
      }

       // Lower Eight columns
      for column in 0..8 {
        //We need four bytes per 4 pixels , because each pixel has a 8-bit color depth
        // thus having 8 bit planes for 4 pixels

        //Get lowest four bits, each bit corresponding to a different pixel, plane 0
        let low_nibble = tile[column] & 0x0F;
        //Get hightest four bits, each bit corresponding to a different pixel, plane 1
        let high_nibble = (tile[column] >> 4) & 0x0F;

        for (i, pixel) in [ 
          Pixel::new(low_nibble & 0x01, high_nibble & 0x01),
          Pixel::new((low_nibble & 0x02) >> 1, (high_nibble & 0x02) >> 1),
          Pixel::new((low_nibble & 0x04) >> 2, (high_nibble & 0x04) >> 2),
          Pixel::new((low_nibble & 0x08) >> 3, (high_nibble & 0x08) >> 3) 
        ].iter().enumerate() {
            let raw_data = pixel.to_rgba();
            let pos = i * self.width + column + offset_y * self.width * 8 + offset_x * 8;
            // if column == 7 && i == 3 {
            //   println!("Last position for lower eight: {}", format!("{:#x}", pos));
            // }
            // if column == 0 && i == 0 {
            //   println!("First position for lower eight: {}", format!("{:#x}", pos));
            // }
            pixel_buffer[pos] = raw_data;
        }
    }
  }
}