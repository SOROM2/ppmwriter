# ppmwriter

## Usage

```rust
use std::fs::File;
use std::io::Write;
use std::path::Path;

use ppmwriter::Writer;

fn main() {
    let name = "image.ppm";
    let mut ppm = Writer::new(69, 35, 2301).unwrap();
    
    for n in 0..ppm.size {
        if n % 2 == 0 {
            ppm.write_pixel(ppm.maxval, 0, ppm.maxval);
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
```
