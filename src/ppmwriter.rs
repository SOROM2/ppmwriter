/**
*     Copyright (C) 2022 Mason Soroka-Gill
*
*     This program is free software: you can redistribute it and/or modify
*     it under the terms of the GNU General Public License as published by
*     the Free Software Foundation, either version 3 of the License, or
*     (at your option) any later version.
*
*     This program is distributed in the hope that it will be useful,
*     but WITHOUT ANY WARRANTY; without even the implied warranty of
*     MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
*     GNU General Public License for more details.
*
*     You should have received a copy of the GNU General Public License
*     along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

#[allow(dead_code)]
#[derive(Debug)]
pub enum Error {
    InvalidSize,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
enum Mode {
    Normal,
    Wide,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Writer {
    pub width: usize,
    pub height: usize,
    pub size: usize,
    pub maxval: u16,
    mode: Mode,
    buffer: Vec<u8>,
}

impl Writer {
    #[allow(dead_code)]
    pub fn new(width: usize, height: usize, maxval: u16) -> Result<Self, Error> {
        if height == 0 || width == 0 || maxval == 0 {
            return Err(Error::InvalidSize);
        }

        let mut mode: Mode = Mode::Normal;
        if maxval > 255 {
            mode = Mode::Wide;
        }

        let size = height * width;
        let mut initial_capacity = size;
        if mode == Mode::Wide {
            initial_capacity *= 2;
        }
        initial_capacity += 16; // for magic and metadata

        let mut buffer: Vec<u8> = Vec::with_capacity(initial_capacity);
        buffer.extend(b"P6\n");
        buffer.extend(format!("{} {} {}\n", width, height, maxval).as_bytes());
        
        Ok(Self {
            width,
            height,
            size,
            maxval,
            mode,
            buffer,
        })
    }

    #[allow(dead_code)]
    pub fn write_pixel(&mut self, r: u16, g: u16, b: u16) {
        if self.mode == Mode::Wide {
            //     mask only the upper 8 bits
            //         0x???? & 0xFF00 = 0x??00
            //     shift the upper 8 bits into the lower position
            //         0x??00 >> 8 = 0xzz 0x??
            let upper = |x: u16| -> u8 { ((x & 0xff00) >> 8) as u8 };
            // lower-byte strategy:
            //     mask only the lower 8 bits
            //         0x???? & 0x00FF = 0x00 0x??
            let lower = |x: u16| -> u8 { (x & 0x00ff) as u8 };

            self.buffer.push(upper(r));
            self.buffer.push(lower(r));

            self.buffer.push(upper(g));
            self.buffer.push(lower(g));

            self.buffer.push(upper(b));
            self.buffer.push(lower(b));
        } else {
            // hard-cast the u16 input to u8
            self.buffer.push(r as u8);
            self.buffer.push(g as u8);
            self.buffer.push(b as u8);
        }
    }

    #[allow(dead_code)]
    pub fn borrow_buffer(&self) -> &Vec<u8> {
        &self.buffer
    }
}

