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

use std::fs::File;
use std::io::Write;
use std::path::Path;

mod ppmwriter;

fn main() {
    let name = "image.ppm";
    let mut ppm = ppmwriter::Writer::new(69, 35, 1).unwrap();
    
    for n in 0..ppm.size {
        if n % 2 == 0 {
            ppm.write_pixel(ppm.maxval, ppm.maxval, ppm.maxval);
        } else {
            ppm.write_pixel(0, 0, 0);
        }
    }

    let path = Path::new(name);
    let mut file = match File::create(path) {
        Err(why) => panic!("Couldn't create {}: {}", path.display(), why),
        Ok(file) => file,
    };

    if let Err(why) = file.write_all(ppm.borrow_buffer()) {
        panic!("Couldn't write to {}: {}", path.display(), why)
    }
}
