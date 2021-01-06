pub struct Pixel {
   pub color: u8
}

/**
 * Each pixel is made of 2 bits, thus having a total of 4 values (out of  16?)
 * For instance, we can use the following color palette until we figure the real colors
 * - 00: blue -> 0x0000FFFF
 * - 01: red -> 0xFF0000FF
 * - 10: yellow -> 0xFFFF00FF
 * - 11: green -> 0x00FF00FF
 **/
  
impl Pixel {
    pub fn new(low_nibble: u8, high_nibble: u8) -> Self {
        Self {
            color: low_nibble << 1 | high_nibble
        }
    }

    pub fn to_rgba(& self) -> u32 {
        match self.color {
            0 => 0x0000FFFF,
            1 => 0xFF0000FF,
            2 => 0xFFFF00FF,
            3 => 0x00FF00FF,
            _ => 0
        }
    }

}